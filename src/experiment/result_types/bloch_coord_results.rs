use crate::experiment::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;

use hdf5::{Group, Result};
use ndarray::{Array2, ArrayD, Axis, IntoDimension, Ix1, IxDyn, SliceInfo, SliceInfoElem};

pub struct BlochCoordResults {
    /// Multi-Dimensional array to store the results of the sweep in
    x_coord_results: ArrayD<f64>,
    y_coord_results: ArrayD<f64>,
    z_coord_results: ArrayD<f64>,
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
        dbg!(&results_dim);
        // Array for results of experiment
        let results: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&results_dim));
        return BlochCoordResults {
            x_coord_results: results.clone(),
            y_coord_results: results.clone(),
            z_coord_results: results,
        };
    }
}

impl ExperimentResult for BlochCoordResults {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> () {
        let (x_coords, y_coords, z_coords): (Array2<f64>, Array2<f64>, Array2<f64>) =
            simulation_result.get_bloch_coords_cart();
        if x_coords.shape() == [1, 1] {
            self.x_coord_results[sweep_parameter_indices.clone().into_dimension()] =
                x_coords[[0, 0]];
            self.y_coord_results[sweep_parameter_indices.clone().into_dimension()] =
                y_coords[[0, 0]];
            self.z_coord_results[sweep_parameter_indices.clone().into_dimension()] =
                z_coords[[0, 0]];
            return;
        }

        let mut slice_info_vec: Vec<SliceInfoElem> = sweep_parameter_indices
            .iter()
            .map(|i| SliceInfoElem::Index(i.clone() as isize))
            .collect();

        if x_coords.shape()[0] != 1 {
            slice_info_vec.push(SliceInfoElem::Slice {
                start: 0,
                end: None,
                step: 1,
            });
        }
        if x_coords.shape()[1] != 1 {
            slice_info_vec.push(SliceInfoElem::Slice {
                start: 0,
                end: None,
                step: 1,
            });
        }

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix1> =
            SliceInfo::try_from(slice_info_vec).unwrap();

        if x_coords.shape()[0] == 1 && x_coords.shape()[1] != 1 {
            self.x_coord_results
                .slice_mut(&slice_info)
                .assign(&x_coords.index_axis_move(Axis(0), 0));
            self.y_coord_results
                .slice_mut(&slice_info)
                .assign(&y_coords.index_axis_move(Axis(0), 0));
            self.z_coord_results
                .slice_mut(&slice_info)
                .assign(&z_coords.index_axis_move(Axis(0), 0));
        } else if x_coords.shape()[0] != 1 && x_coords.shape()[1] == 1 {
            self.x_coord_results
                .slice_mut(&slice_info)
                .assign(&x_coords.index_axis_move(Axis(1), 0));
            self.y_coord_results
                .slice_mut(&slice_info)
                .assign(&y_coords.index_axis_move(Axis(1), 0));
            self.z_coord_results
                .slice_mut(&slice_info)
                .assign(&z_coords.index_axis_move(Axis(1), 0));
        } else {
            self.x_coord_results
                .slice_mut(&slice_info)
                .assign(&x_coords);
            self.y_coord_results
                .slice_mut(&slice_info)
                .assign(&y_coords);
            self.z_coord_results
                .slice_mut(&slice_info)
                .assign(&z_coords);
        }
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        let bloch_coords_group: Group = group.create_group("bloch_coords")?;
        let builder = bloch_coords_group.new_dataset_builder();
        let _ds = builder
            .clone()
            .with_data(&self.x_coord_results)
            .create("x_coords")?;
        let _ds = builder
            .clone()
            .with_data(&self.y_coord_results)
            .create("y_coords")?;
        let _ds = builder
            .with_data(&self.z_coord_results)
            .create("z_coords")?;
        return Ok(());
    }
}
