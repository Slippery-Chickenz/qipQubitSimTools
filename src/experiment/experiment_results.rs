use std::rc::Rc;

use super::SweepParameter;
use crate::simulation::SimulationResults;

use super::probability_results::ProbabilityResults;
use super::bloch_coord_results::BlochCoordResults;

use serde_json::{Map, Value};
use hdf5::{Group, Result};

pub trait ExperimentResult {
    fn add_simulation_result(&mut self, sweep_parameter_indices: &Vec<usize>, simulation_result: &SimulationResults) -> ();
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
    pub fn from_json(json_values: &Map<String, Value>, sweep_parameters: Rc<Vec<SweepParameter>>) -> ExperimentResults {

        let mut results: Vec<Box<dyn ExperimentResult>> = vec![];

        if json_values.contains_key("state") {
            results.push(Box::new(ProbabilityResults::from_json(json_values, Rc::clone(&sweep_parameters))));
        }
        if json_values.contains_key("bloch_coords") {
            if json_values["bloch_coords"].as_bool().unwrap() {
                results.push(Box::new(BlochCoordResults::from_json(json_values, Rc::clone(&sweep_parameters))));
            }
        }

        return ExperimentResults { results: results, sweep_parameters: sweep_parameters }
    }
    pub fn add_simulation_result(&mut self, sweep_parameter_indices: &Vec<usize>, simulation_result: &SimulationResults) -> () {
        for result in &mut self.results {
            result.add_simulation_result(sweep_parameter_indices, simulation_result);
        }
        return;
    }
    pub fn save(&self, mut filename: String) -> Result<()> {

        // Add the .h5 extension to the filename
        filename.push_str(".h5");

        // Open an HDF5 file under the given name and make a parameters group
        let file = hdf5::File::create(filename)?;
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

        let results_group: Group = file.create_group("results")?;
        for result in &self.results {
            result.save(&results_group)?;
        }
        return Ok(());
    }
}
