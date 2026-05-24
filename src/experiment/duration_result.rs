use super::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;

use hdf5::{Group, Result};
use ndarray::{ArrayD, IntoDimension, IxDyn};

pub struct DurationResult {
    /// Multi-Dimensional array to store the results of the sweep in
    results: ArrayD<f64>,
}

impl DurationResult {
    pub fn from_json(results_dim: Vec<usize>, _num_samples: usize) -> DurationResult {
        // Array for results of experiment
        let results: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&results_dim));
        return DurationResult { results };
    }
}

impl ExperimentResult for DurationResult {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> () {
        self.results[sweep_parameter_indices.clone().into_dimension()] =
            simulation_result.get_duration();
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.results).create("durations")?;
        return Ok(());
    }
}
