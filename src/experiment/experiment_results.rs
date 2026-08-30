use std::rc::Rc;

use super::SweepParameter;
use crate::{
    experiment::time_results::TimeResults,
    simulation::{Circuit, SimulationResultGetter},
};

use super::adiabaticity_results::AdiabaticityResults;
use super::bloch_coord_results::BlochCoordResults;
use super::duration_result::DurationResult;
use super::hamiltonian_results::HamiltonianResults;
use super::probability_results::ProbabilityResults;
use super::eigenstate_results::EigenstateResults;
use super::waveform_saver::WaveformSaver;

use hdf5::{Group, Result};
use serde_json::{Map, Value};

pub trait ExperimentResult {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> ();
    fn save(&self, group: &Group) -> Result<()>;
}

impl std::fmt::Debug for dyn ExperimentResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct ExperimentResults {
    results: Vec<Box<dyn ExperimentResult>>,
    sweep_parameters: Rc<Vec<SweepParameter>>,
    waveform_saver: Option<WaveformSaver>,
}

impl ExperimentResults {
    pub fn from_json(
        json_values: &Map<String, Value>,
        sweep_parameters: Rc<Vec<SweepParameter>>,
    ) -> (ExperimentResults, bool, bool) {
        // Vector to hold the dimensions of the results
        let mut results_dim: Vec<usize> = vec![];
        // Loop over the sweep parameters and add the len of the values as the length of the dimension
        for sweep_parameter in &*sweep_parameters {
            results_dim.push(sweep_parameter.values_len());
        }

        // Number of samples per simulation
        let num_samples: usize = json_values["num_samples"].as_u64().unwrap() as usize;

        // List of all the results from the simulations to store
        let mut results: Vec<Box<dyn ExperimentResult>> = vec![];

        // Add a result to store the total duration of each simulation
        results.push(Box::new(DurationResult::from_json(
            results_dim.clone(),
            num_samples,
        )));

        let mut save_waveform: bool = false;
        if json_values.contains_key("waveform") {
            save_waveform = json_values["waveform"].as_bool().unwrap();
        }

        let mut save_hamiltonians: bool = false;
        if json_values.contains_key("hamiltonians") {
            if json_values["hamiltonians"].as_bool().unwrap() {
                results.push(Box::new(HamiltonianResults::from_json(
                    results_dim.clone(),
                    num_samples,
                )));
                save_hamiltonians = true;
            }
        }
        if json_values.contains_key("state") {
            results.push(Box::new(ProbabilityResults::from_json(
                results_dim.clone(),
                num_samples,
            )));
        }
        if json_values.contains_key("times") {
            results.push(Box::new(TimeResults::from_json(
                results_dim.clone(),
                num_samples,
            )));
        }
        if json_values.contains_key("bloch_coords") {
            if json_values["bloch_coords"].as_bool().unwrap() {
                results.push(Box::new(BlochCoordResults::from_json(
                    results_dim.clone(),
                    num_samples,
                )));
            }
        }
        if json_values.contains_key("adiabaticity") {
            if json_values["adiabaticity"].as_bool().unwrap() {
                results.push(Box::new(AdiabaticityResults::from_json(
                    results_dim.clone(),
                    num_samples,
                )));
                save_hamiltonians = true;
            }
        }
        if json_values.contains_key("eigenstates") {
            if json_values["eigenstates"].as_bool().unwrap() {
                results.push(Box::new(EigenstateResults::from_json(
                    results_dim.clone(),
                    num_samples,
                )));
                save_hamiltonians = true;
            }
        }

        return (
            ExperimentResults {
                results: results,
                sweep_parameters: sweep_parameters,
                waveform_saver: Option::None,
            },
            save_hamiltonians,
            save_waveform,
        );
    }
    pub fn save_circuit(&mut self, circuit: Circuit) -> () {
        self.waveform_saver = Option::Some(WaveformSaver::from_circuit(circuit));
        return;
    }
    pub fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> () {
        for result in &mut self.results {
            result.add_simulation_result(sweep_parameter_indices, simulation_result);
        }
        return;
    }
    pub fn save(&self, filename: &str) -> Result<()> {
        // Open an HDF5 file under the given name
        let file = hdf5::File::create(filename.to_string() + ".h5")?;

        // Loop through all the results and save them and collect the duration of all of them
        let results_group: Group = file.create_group("results")?;
        for result in &self.results {
            result.save(&results_group)?;
        }

        // Make a parameters group
        let group = file.create_group("parameters")?;

        // Loop over all the swept parameters in this experiment
        for (i, swept_parameter) in self.sweep_parameters.iter().enumerate() {
            // Construct a builder for this parameter
            let builder = group.new_dataset_builder();
            // Build a dataset with the values this parameter is swept over
            let parameter_ds = builder
                .with_data(swept_parameter.get_values())
                .create(swept_parameter.get_full_path().as_str())?;
            // Create at attribute for this parameter and write which number axis this parameter is
            let attr = parameter_ds.new_attr::<usize>().shape([1]).create("axis")?;
            attr.write(&[i])?;
        }

        // If there is a waveform to save then make a group and save it
        if let Some(waveform_saver) = &self.waveform_saver {
            let group = file.create_group("waveform")?;
            waveform_saver.save(&group)?;
        }
        return Ok(());
    }
}
