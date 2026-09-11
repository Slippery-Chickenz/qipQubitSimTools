use std::rc::Rc;

use super::SweepParameter;
use crate::{
    experiment::time_results::TimeResults,
    simulation::{Circuit, SimulationResultGetter},
};

use super::adiabaticity_results::AdiabaticityResults;
use super::bloch_coord_results::BlochCoordResults;
use super::duration_result::DurationResult;
use super::eigenstate_results::EigenstateResults;
use super::hamiltonian_results::HamiltonianResults;
use super::measurement_results::MeasurementResults;
use super::probability_results::ProbabilityResults;
use super::state_results::StateResults;
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
        mut json_values: Map<String, Value>,
        sweep_parameters: Rc<Vec<SweepParameter>>,
        num_samples: usize,
    ) -> (ExperimentResults, bool, bool) {
        // Vector to hold the dimensions of the results
        let results_dim: Vec<usize> =
            Vec::from_iter((*sweep_parameters).iter().map(|x| x.values_len()));

        // Whether to save the waveform
        let save_waveform: bool =
            serde_json::from_value(json_values.remove("waveform").unwrap_or_default())
                .unwrap_or(false);
        // Whether to save the hamiltonian at each sample
        let mut save_hamiltonians: bool = false;

        // List of all the results from the simulations to store
        let mut results: Vec<Box<dyn ExperimentResult>> = vec![];

        // Loop over the outputs to save and get a result for each of them
        for (key, value) in json_values.into_iter() {
            // If it is a bool and false then don't include it otherwise try to
            if !value.as_bool().unwrap_or(true) {
                continue;
            }
            // Get the result and add it to the vec
            let (result, save_hamiltonian): (Box<dyn ExperimentResult>, bool) =
                ExperimentResults::get_result_from_json(
                    key.as_str(),
                    value,
                    results_dim.clone(),
                    num_samples,
                );
            if save_hamiltonian {
                save_hamiltonians = true;
            }
            results.push(result);
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
    fn get_result_from_json(
        name: &str,
        json_value: Value,
        results_dim: Vec<usize>,
        num_samples: usize,
    ) -> (Box<dyn ExperimentResult>, bool) {
        match name {
            "state" => (
                Box::new(StateResults::from_json(results_dim.clone(), num_samples)),
                false,
            ),
            "duration" => (
                Box::new(DurationResult::from_json(results_dim.clone(), num_samples)),
                false,
            ),
            "measurement" => (
                Box::new(MeasurementResults::from_json(
                    results_dim.clone(),
                    num_samples,
                    json_value,
                )),
                false,
            ),
            "probability" => (
                Box::new(ProbabilityResults::from_json(
                    results_dim.clone(),
                    num_samples,
                    json_value,
                )),
                false,
            ),
            "bloch_coords" => (
                Box::new(BlochCoordResults::from_json(
                    results_dim.clone(),
                    num_samples,
                )),
                false,
            ),
            "times" => (
                Box::new(TimeResults::from_json(results_dim.clone(), num_samples)),
                false,
            ),
            "hamiltonians" => (
                Box::new(HamiltonianResults::from_json(
                    results_dim.clone(),
                    num_samples,
                )),
                true,
            ),
            "adiabaticity" => (
                Box::new(AdiabaticityResults::from_json(
                    results_dim.clone(),
                    num_samples,
                )),
                true,
            ),
            "eigenstates" => (
                Box::new(EigenstateResults::from_json(
                    results_dim.clone(),
                    num_samples,
                )),
                true,
            ),
            _ => panic!("Invalid result name! {}", name),
        }
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
                .create(swept_parameter.get_full_path().as_str())
                .unwrap();
            // Create at attribute for this parameter and write which number axis this parameter is
            let attr = parameter_ds
                .new_attr::<usize>()
                .shape([1])
                .create("axis")
                .unwrap();
            attr.write(&[i]).unwrap();
        }

        // If there is a waveform to save then make a group and save it
        if let Some(waveform_saver) = &self.waveform_saver {
            let group = file.create_group("waveform")?;
            waveform_saver.save(&group)?;
        }
        return Ok(());
    }
}
