use crate::experiment::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;

use hdf5::{Group, Result};
use ndarray::{Array3, ArrayD, Axis, Ix1, Ix2, Ix3, IxDyn, SliceInfo, SliceInfoElem};

pub struct BlochCoordResults {
    /// Multi-Dimensional array to store the results of the sweep in
    coord_results: ArrayD<f64>,
}

impl BlochCoordResults {
    pub fn from_json(
        mut results_dim: Vec<usize>,
        num_shots: usize,
        num_samples: usize,
    ) -> BlochCoordResults {
        if num_shots > 1 {
            results_dim.push(num_shots);
        }
        // If there is more than one sample add that as a dimension
        if num_samples > 1 {
            results_dim.push(num_samples);
        }

        results_dim.push(3);
        // Array for results of experiment
        let results: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&results_dim));
        return BlochCoordResults {
            coord_results: results,
        };
    }
}

impl ExperimentResult for BlochCoordResults {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &mut dyn SimulationResultGetter,
    ) -> () {
        let coords: Array3<f64> = simulation_result.get_bloch_coords_cart();

        let mut slice_info_vec: Vec<SliceInfoElem> = sweep_parameter_indices
            .iter()
            .map(|i| SliceInfoElem::Index(i.clone() as isize))
            .collect();

        // Add a slice if there is more than 1 shot
        if coords.shape()[0] != 1 {
            slice_info_vec.push(SliceInfoElem::Slice {
                start: 0,
                end: None,
                step: 1,
            });
        }
        // Add a slice if there is more than 1 sample
        if coords.shape()[1] != 1 {
            slice_info_vec.push(SliceInfoElem::Slice {
                start: 0,
                end: None,
                step: 1,
            });
        }
        // Add a slice for x, y, and z
        slice_info_vec.push(SliceInfoElem::Slice {
            start: 0,
            end: None,
            step: 1,
        });

        if coords.shape()[0] == 1 && coords.shape()[1] == 1 {
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix1> =
                SliceInfo::try_from(slice_info_vec).unwrap();
            self.coord_results.slice_mut(&slice_info).assign(
                &coords
                    .index_axis_move(Axis(0), 0)
                    .index_axis_move(Axis(0), 0),
            );
        } else if coords.shape()[0] == 1 && coords.shape()[1] != 1 {
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
                SliceInfo::try_from(slice_info_vec).unwrap();
            self.coord_results
                .slice_mut(&slice_info)
                .assign(&coords.index_axis_move(Axis(0), 0));
        } else if coords.shape()[0] != 1 && coords.shape()[1] == 1 {
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
                SliceInfo::try_from(slice_info_vec).unwrap();
            self.coord_results
                .slice_mut(&slice_info)
                .assign(&coords.index_axis_move(Axis(1), 0));
        } else {
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix3> =
                SliceInfo::try_from(slice_info_vec).unwrap();
            self.coord_results.slice_mut(&slice_info).assign(&coords);
        }
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        let builder = group.new_dataset_builder();
        let _ds = builder
            .clone()
            .with_data(&self.coord_results)
            .create("bloch_coords")?;
        return Ok(());
    }
}
