use std::rc::Rc;

use crate::simulation::SimulationResults;

pub use super::SweepParameter;

use ndarray::{Array1, ArrayD, IxDyn, Ix1, SliceInfo, SliceInfoElem, IntoDimension};
use serde_json::{Map, Value};
use hdf5::Result;

pub trait ExperimentResult {
    fn add_simulation_result(&mut self, sweep_parameter_indices: &Vec<usize>, simulation_result: SimulationResults) -> ();
    fn save(&self, filename: String) -> Result<()>;
}

impl std::fmt::Debug for dyn ExperimentResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub fn get_experiment_result_from_json(json_values: &Map<String, Value>, sweep_parameters: Rc<Vec<SweepParameter>>) -> Box<dyn ExperimentResult> {
    return Box::new(ProbabilityResults::from_json(json_values, sweep_parameters));
}

pub struct ProbabilityResults {
    /// Multi-Dimensional array to store the results of the sweep in
    results: ArrayD<f64>,
    sweep_parameters: Rc<Vec<SweepParameter>>,
}

impl ProbabilityResults {
    pub fn from_json(
        json_values: &Map<String, Value>,
        sweep_parameters: Rc<Vec<SweepParameter>>,
    ) -> ProbabilityResults {

        // Vector to hold the dimensions of the results
        let mut results_dim: Vec<usize> = vec![];
        // Loop over the sweep parameters and add the len of the values as the length of the dimension
        for sweep_parameter in &*sweep_parameters {
            results_dim.push(sweep_parameter.values_len());
        }

        if json_values["num_samples"].as_u64().unwrap() > 1 {
            results_dim.push(json_values["num_samples"].as_u64().unwrap() as usize);
        }

        // Array for results of experiment
        let results: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&results_dim));
        return ProbabilityResults { results, sweep_parameters: sweep_parameters};
    }
}

impl ExperimentResult for ProbabilityResults {
    fn add_simulation_result(&mut self, sweep_parameter_indices: &Vec<usize>, simulation_result: SimulationResults) -> () {

        let probabilities: Array1<f64> = simulation_result.get_probabilities();
        if probabilities.len() == 1 {
            self.results[sweep_parameter_indices.clone().into_dimension()] = probabilities[0];
            return;
        }

        let mut slice_info_vec: Vec<SliceInfoElem> = vec![];

        for index in sweep_parameter_indices {
            slice_info_vec.push(SliceInfoElem::Index(index.clone() as isize));
        }

        slice_info_vec.push(SliceInfoElem::Slice { start: 0, end: None, step: 1 });


        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix1> = SliceInfo::try_from(slice_info_vec).unwrap();
        self.results.slice_mut(slice_info).assign(&probabilities);
        return;
    }
    fn save(&self, mut filename: String) -> Result<()> {
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
        // Make a builder and put the results data set into the file
        let builder = file.new_dataset_builder();
        let _ds = builder.with_data(&self.results).create("results")?;
        return Ok(());
    }
}
