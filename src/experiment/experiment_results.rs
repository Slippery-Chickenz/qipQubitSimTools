use std::rc::Rc;

use super::SweepParameter;
use crate::simulation::SimulationResults;

use super::bloch_coord_results::BlochCoordResults;
use super::duration_result::DurationResult;
use super::probability_results::ProbabilityResults;

use hdf5::{Group, Result};
use serde_json::{Map, Value};

pub trait ExperimentResult {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &SimulationResults,
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
}

impl ExperimentResults {
    pub fn from_json(
        json_values: &Map<String, Value>,
        sweep_parameters: Rc<Vec<SweepParameter>>,
    ) -> ExperimentResults {
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

        if json_values.contains_key("state") {
            results.push(Box::new(ProbabilityResults::from_json(
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

        return ExperimentResults {
            results: results,
            sweep_parameters: sweep_parameters,
        };
    }
    pub fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &SimulationResults,
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
        return Ok(());
    }
}
