use crate::experiment::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;

use hdf5::{Group, Result};
use ndarray::{Array3, ArrayD, Axis, Ix2, Ix3, IxDyn, SliceInfo, SliceInfoElem};
use num_complex::Complex64;

pub struct StateResults {
    /// Multi-Dimensional array to store the results of the sweep in
    states: ArrayD<Complex64>,
}

impl StateResults {
    pub fn from_json(
        mut results_dim: Vec<usize>,
        num_shots: usize,
        num_samples: usize,
    ) -> StateResults {
        if num_shots > 1 {
            results_dim.push(num_shots);
        }
        if num_samples > 1 {
            results_dim.push(num_samples);
        }
        results_dim.push(2);
        // Set the eigen value array and the eigen state array
        let states: ArrayD<Complex64> = ArrayD::<Complex64>::zeros(IxDyn(&results_dim));
        return StateResults { states: states };
    }
}

impl ExperimentResult for StateResults {
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
        let states: Array3<Complex64> = simulation_result.get_states();

        // Add a full slice to account for the dimension corresponding to the state
        slice_info_vec.push(SliceInfoElem::Slice {
            start: 0,
            end: None,
            step: 1,
        });

        // Add a dimension depending on if there is more than 1 shot/sample
        if states.shape()[0] != 1 {
            slice_info_vec.push(SliceInfoElem::Slice {
                start: 0,
                end: None,
                step: 1,
            });
        }
        if states.shape()[1] != 1 {
            slice_info_vec.push(SliceInfoElem::Slice {
                start: 0,
                end: None,
                step: 1,
            });
        }

        // Assign the results depending on if there is more than 1 shot/sample
        if states.shape()[0] == 1 && states.shape()[1] != 1 {
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
                SliceInfo::try_from(slice_info_vec.clone()).unwrap();
            self.states
                .slice_mut(slice_info)
                .assign(&states.index_axis_move(Axis(0), 0));
        } else if states.shape()[0] != 1 && states.shape()[1] == 1 {
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
                SliceInfo::try_from(slice_info_vec.clone()).unwrap();
            self.states
                .slice_mut(slice_info)
                .assign(&states.index_axis_move(Axis(1), 0));
        } else {
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix3> =
                SliceInfo::try_from(slice_info_vec.clone()).unwrap();
            self.states.slice_mut(slice_info).assign(&states);
        }
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
