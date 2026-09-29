use crate::experiment::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;

use hdf5::{Group, Result};
use ndarray::{Array2, Array3, ArrayD, Axis, Ix2, Ix3, IxDyn, SliceInfo, SliceInfoElem};
use num_complex::Complex64;

pub struct LarmorResults {
    /// Multi-Dimensional array to store the results of the sweep in
    larmors: ArrayD<Array2<f64>>,
}

impl LarmorResults {
    pub fn from_json(
        results_dim: Vec<usize>,
    ) -> LarmorResults {
        // Set the eigen value array and the eigen state array
        let larmors: ArrayD<Array2<f64>> = ArrayD::<Array2<f64>>::from_elem(IxDyn(&results_dim), Array2::<f64>::zeros([0, 0]));
        return LarmorResults { larmors: larmors };
    }
}

impl ExperimentResult for LarmorResults {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> () {
        // Get the slice information for the results at these parameters
        let mut slice_info_vec: Vec<SliceInfoElem> = sweep_parameter_indices
            .iter()
            .map(|i| SliceInfoElem::Index(i.clone() as isize))
            .collect();

        // Get the states from the simulation run
        let larmor_values: Array2<f64> = simulation_result.take_larmors();

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
            SliceInfo::try_from(slice_info_vec.clone()).unwrap();
        self.larmors.slice_mut(slice_info).assign(&larmor_values);
        self.larmors[slice_info_vec] = larmor_values;
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.states).create("states")?;
        return Ok(());
    }
}
