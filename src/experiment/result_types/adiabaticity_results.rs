use crate::experiment::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;

use hdf5::{Group, Result};
use ndarray::{Array2, ArrayD, IntoDimension, Ix1, IxDyn, SliceInfo, SliceInfoElem};

pub struct AdiabaticityResults {
    /// Multi-Dimensional array to store the results of the sweep in
    results: ArrayD<f64>,
}

impl AdiabaticityResults {
    pub fn from_json(
        mut results_dim: Vec<usize>,
        num_shots: usize,
        num_samples: usize,
    ) -> AdiabaticityResults {
        if num_shots > 1 {
            results_dim.push(num_shots);
        }
        if num_samples > 1 {
            results_dim.push(num_samples);
        }
        // Array for results of experiment
        let results: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&results_dim));
        return AdiabaticityResults { results };
    }
}

impl ExperimentResult for AdiabaticityResults {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> () {
        let probabilities: Array2<f64> = simulation_result.get_probabilities();
        if probabilities.shape()[0] == 1 {
            self.results[sweep_parameter_indices.clone().into_dimension()] = probabilities[[0, 0]];
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
        self.results.slice_mut(slice_info).assign(&probabilities);
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.results).create("adiabaticity")?;
        return Ok(());
    }
}
