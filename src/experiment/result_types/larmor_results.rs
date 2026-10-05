use crate::experiment::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;

use hdf5::{Group, Result};
use ndarray::{Array1, Array2, Array3, ArrayD, Axis, IntoDimension, Ix1, Ix2, Ix3, IxDyn, ShapeArg, SliceInfo, SliceInfoElem};
use num_complex::Complex64;

pub struct LarmorResults {
    /// Multi-Dimensional array to store the results of the sweep in
    larmors: ArrayD<Array1<f64>>,
    /// Number of shots
    num_shots: usize,
    /// Maximum number of larmors stored in a single shot
    max_iterations: usize,
}

impl LarmorResults {
    pub fn from_json(
        results_dim: Vec<usize>,
        num_shots: usize,
    ) -> LarmorResults {
        // Set the eigen value array and the eigen state array
        let larmors: ArrayD<Array1<f64>> = ArrayD::<Array1<f64>>::from_elem(IxDyn(&results_dim), Array1::<f64>::zeros(0));
        return LarmorResults { larmors: larmors, num_shots: num_shots, max_iterations: 0 };
    }
}

impl ExperimentResult for LarmorResults {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &mut dyn SimulationResultGetter,
    ) -> () {
        // Get the slice information for the results at these parameters
        // let mut slice_info_vec: Vec<SliceInfoElem> = sweep_parameter_indices
        //     .iter()
        //     .map(|i| SliceInfoElem::Index(i.clone() as isize))
        //     .collect();

        // Get the states from the simulation run
        let larmor_values: Array1<f64> = simulation_result.take_larmors().into_flat();

        // Check if this is more iterations and previous max
        if larmor_values.len() > self.max_iterations {
            self.max_iterations = larmor_values.len();
        }

        // let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
        //     SliceInfo::try_from(slice_info_vec.clone()).unwrap();
        // self.larmors.slice_mut(slice_info).assign(&larmor_values);
        self.larmors[sweep_parameter_indices.as_slice().into_dimension()] = larmor_values;
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {


        let mut padded_dimensions: Vec<usize> = self.larmors.shape().to_vec();
        padded_dimensions.push(self.max_iterations);
        // padded_dimensions.push(self.max_iterations);

        let mut padded_larmors: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&padded_dimensions));

        // Vector of indicies in the dynamic array to loop over
        let mut sim_indicies: Vec<usize> = vec![0; self.larmors.shape().len()];

        // Loop over the saved data and put it into the padded array
        for (_i, larmor_values) in self.larmors.iter().enumerate() {
            // Get the slice information for the results at these parameters
            let mut slice_info_vec: Vec<SliceInfoElem> = sim_indicies
                .iter()
                .map(|i| SliceInfoElem::Index(i.clone() as isize))
                .collect();
            slice_info_vec.push(SliceInfoElem::Slice {
                start: 0,
                end: Some(larmor_values.len() as isize),
                step: 1,
            });
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix1> =
                SliceInfo::try_from(slice_info_vec.clone()).unwrap();
            padded_larmors.slice_mut(slice_info).assign(&larmor_values);

            for j in 0..sim_indicies.len() {
                // Increase the parameter index
                sim_indicies[j] += 1;
                // If the parameter index that was just increased is the last one for that
                // parameter then reset it and go onto the next parameter index
                if sim_indicies[j] >= padded_dimensions[j] {
                    sim_indicies[j] = 0;
                } else {
                    // If it was not the last one for that parameter then just break and only
                    // increase that one
                    break;
                }
            }

        }

        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&padded_larmors).create("larmors")?;
        return Ok(());
    }
}
