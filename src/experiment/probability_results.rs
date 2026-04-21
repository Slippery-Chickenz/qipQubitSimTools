use std::rc::Rc;

pub use super::SweepParameter;
use super::experiment_results::ExperimentResult;
use crate::simulation::SimulationResults;

use ndarray::{Array1, ArrayD, IxDyn, Ix1, SliceInfo, SliceInfoElem, IntoDimension};
use serde_json::{Map, Value};
use hdf5::{Group, Result};

pub struct ProbabilityResults {
    /// Multi-Dimensional array to store the results of the sweep in
    results: ArrayD<f64>,
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
        return ProbabilityResults { results };
    }
}

impl ExperimentResult for ProbabilityResults {
    fn add_simulation_result(&mut self, sweep_parameter_indices: &Vec<usize>, simulation_result: &SimulationResults) -> () {

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
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.results).create("probabilities")?;
        return Ok(());
    }
}
