use super::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;

use hdf5::{Group, Result};
use ndarray::{Array3, ArrayD, Axis, Ix2, Ix3, IxDyn, SliceInfo, SliceInfoElem};
use num_complex::Complex64;

pub struct HamiltonianResults {
    /// Multi-Dimensional array to store the results of the sweep in
    results: ArrayD<Complex64>,
}

impl HamiltonianResults {
    pub fn from_json(mut results_dim: Vec<usize>, num_samples: usize) -> HamiltonianResults {
        if num_samples > 1 {
            results_dim.push(num_samples);
        }
        results_dim.push(2);
        results_dim.push(2);
        // Array for results of experiment
        let results: ArrayD<Complex64> = ArrayD::<Complex64>::zeros(IxDyn(&results_dim));
        return HamiltonianResults { results };
    }
}

impl ExperimentResult for HamiltonianResults {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> () {
        let hamiltonians: Array3<Complex64> = simulation_result.get_hamiltonians().clone();

        let mut slice_info_vec: Vec<SliceInfoElem> = vec![];

        for index in sweep_parameter_indices {
            slice_info_vec.push(SliceInfoElem::Index(index.clone() as isize));
        }
        slice_info_vec.push(SliceInfoElem::Slice {
            start: 0,
            end: None,
            step: 1,
        });
        slice_info_vec.push(SliceInfoElem::Slice {
            start: 0,
            end: None,
            step: 1,
        });

        if hamiltonians.shape()[0] == 1 {
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
                SliceInfo::try_from(slice_info_vec).unwrap();
            self.results
                .slice_mut(slice_info)
                .assign(&(hamiltonians.index_axis(Axis(0), 0)));
            return;
        }

        slice_info_vec.push(SliceInfoElem::Slice {
            start: 0,
            end: None,
            step: 1,
        });
        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix3> =
            SliceInfo::try_from(slice_info_vec).unwrap();

        self.results.slice_mut(slice_info).assign(&hamiltonians);
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.results).create("hamiltonians")?;
        return Ok(());
    }
}
