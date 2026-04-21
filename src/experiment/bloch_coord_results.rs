use std::{rc::Rc};

pub use super::SweepParameter;
use super::experiment_results::ExperimentResult;
use crate::simulation::SimulationResults;

use ndarray::{Array1, ArrayD, IxDyn, Ix1, SliceInfo, SliceInfoElem, IntoDimension};
use serde_json::{Map, Value};
use hdf5::{Group, Result};

pub struct BlochCoordResults {
    /// Multi-Dimensional array to store the results of the sweep in
    x_coord_results: ArrayD<f64>,
    y_coord_results: ArrayD<f64>,
    z_coord_results: ArrayD<f64>,
}

impl BlochCoordResults {
    pub fn from_json(
        json_values: &Map<String, Value>,
        sweep_parameters: Rc<Vec<SweepParameter>>,
    ) -> BlochCoordResults {

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
        return BlochCoordResults { x_coord_results: results.clone(), y_coord_results: results.clone(), z_coord_results: results };
    }
}

impl ExperimentResult for BlochCoordResults {
    fn add_simulation_result(&mut self, sweep_parameter_indices: &Vec<usize>, simulation_result: &SimulationResults) -> () {

        let (x_coords, y_coords, z_coords): (Array1<f64>, Array1<f64>, Array1<f64>) = simulation_result.get_bloch_coords_cart();
        if x_coords.len() == 1 {
            self.x_coord_results[sweep_parameter_indices.clone().into_dimension()] = x_coords[0];
            self.y_coord_results[sweep_parameter_indices.clone().into_dimension()] = y_coords[0];
            self.z_coord_results[sweep_parameter_indices.clone().into_dimension()] = z_coords[0];
            return;
        }

        let mut slice_info_vec: Vec<SliceInfoElem> = vec![];

        for index in sweep_parameter_indices {
            slice_info_vec.push(SliceInfoElem::Index(index.clone() as isize));
        }

        slice_info_vec.push(SliceInfoElem::Slice { start: 0, end: None, step: 1 });


        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix1> = SliceInfo::try_from(slice_info_vec).unwrap();
        self.x_coord_results.slice_mut(&slice_info).assign(&x_coords);
        self.y_coord_results.slice_mut(&slice_info).assign(&y_coords);
        self.z_coord_results.slice_mut(slice_info).assign(&z_coords);
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        let bloch_coords_group: Group = group.create_group("bloch_coords")?;
        let builder = bloch_coords_group.new_dataset_builder();
        let _ds = builder.clone().with_data(&self.x_coord_results).create("x_coords")?;
        let _ds = builder.clone().with_data(&self.y_coord_results).create("y_coords")?;
        let _ds = builder.with_data(&self.z_coord_results).create("z_coords")?;
        return Ok(());
    }
}
