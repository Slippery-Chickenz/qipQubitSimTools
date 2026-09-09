use super::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;
use crate::utils::get_state_from_json;

use hdf5::{Group, Result};
use ndarray::{Array1, ArrayD, IntoDimension, Ix1, IxDyn, SliceInfo, SliceInfoElem};
use num_complex::Complex64;
use serde_json::Value;

pub struct ProbabilityResults {
    /// Multi-Dimensional array to store the results of the sweep in
    probabilities: ArrayD<f64>,
    /// State that the probabilities are to be in
    state: Array1<Complex64>,
}

impl ProbabilityResults {
    pub fn from_json(
        mut results_dim: Vec<usize>,
        num_samples: usize,
        json_values: &Value,
    ) -> ProbabilityResults {
        if num_samples > 1 {
            results_dim.push(num_samples);
        }

        // Get the state to find the probability for
        let state: Array1<Complex64> = get_state_from_json(json_values);

        // Array for results of experiment
        let results: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&results_dim));
        return ProbabilityResults {
            probabilities: results,
            state: state,
        };
    }
}

impl ExperimentResult for ProbabilityResults {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> () {
        let probabilities: Array1<f64> = simulation_result.get_state_probabilities(&self.state);
        if probabilities.len() == 1 {
            self.probabilities[sweep_parameter_indices.clone().into_dimension()] = probabilities[0];
            return;
        }

        let mut slice_info_vec: Vec<SliceInfoElem> = vec![];

        for index in sweep_parameter_indices {
            slice_info_vec.push(SliceInfoElem::Index(index.clone() as isize));
        }

        slice_info_vec.push(SliceInfoElem::Slice {
            start: 0,
            end: None,
            step: 1,
        });

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix1> =
            SliceInfo::try_from(slice_info_vec).unwrap();
        self.probabilities
            .slice_mut(slice_info)
            .assign(&probabilities);
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder
            .with_data(&self.probabilities)
            .create("probabilities")?;
        return Ok(());
    }
}
