use super::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;
use crate::utils::get_state_from_json;

use hdf5::{Group, Result};
use ndarray::{Array1, ArrayD, IntoDimension, Ix1, IxDyn, SliceInfo, SliceInfoElem};
use rand::RngExt;
use num_complex::Complex64;
use serde_json::Value;

pub struct MeasurementResults {
    /// Multi-Dimensional array to store the results of the sweep in
    measurements: ArrayD<bool>,
    /// State that the probabilities are to be in
    state: Array1<Complex64>,
}

impl MeasurementResults {
    pub fn from_json(mut results_dim: Vec<usize>, num_samples: usize, json_values: &Value) -> MeasurementResults {
        if num_samples > 1 {
            results_dim.push(num_samples);
        }

        // Get the state to find the probability for
        let state: Array1<Complex64> = get_state_from_json(json_values);

        // Array for results of experiment
        let results: ArrayD<bool> =
            ArrayD::<bool>::from_shape_simple_fn(IxDyn(&results_dim), || false);
        return MeasurementResults {
            measurements: results,
            state: state
        };
    }
}

impl ExperimentResult for MeasurementResults {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> () {
        let probabilities: Array1<f64> = simulation_result.get_state_probabilities(&self.state);
        let mut rng = rand::rng();
        let mut measurement_values: Array1<bool> =
            Array1::<bool>::from_shape_simple_fn(probabilities.shape()[0], || false);
        for (i, measurement) in measurement_values.iter_mut().enumerate() {
            *measurement = rng.random_bool(probabilities[i].max(0.).min(1.));
        }

        if probabilities.len() == 1 {
            self.measurements[sweep_parameter_indices.clone().into_dimension()] =
                measurement_values[0];
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
        self.measurements
            .slice_mut(slice_info)
            .assign(&measurement_values);
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder
            .with_data(&self.measurements)
            .create("measurements")?;
        return Ok(());
    }
}
