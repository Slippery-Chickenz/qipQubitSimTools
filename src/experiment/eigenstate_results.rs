use super::experiment_results::ExperimentResult;
use crate::simulation::SimulationResultGetter;

use hdf5::{Group, Result};
use ndarray::{Array2, Array3, ArrayD, Axis, Ix2, Ix3, IxDyn, SliceInfo, SliceInfoElem};
use ndarray_linalg::{Eigh, UPLO};
use num_complex::Complex64;

pub struct EigenstateResults {
    /// Multi-Dimensional array to store the eigenstates of the sweep in
    eigenstates: ArrayD<Complex64>,
    /// Multi-Dimensional array to store the eigenenergies of the sweep in
    eigenenergies: ArrayD<f64>,
}

impl EigenstateResults {
    pub fn from_json(mut results_dim: Vec<usize>, num_samples: usize) -> EigenstateResults {
        if num_samples > 1 {
            results_dim.push(num_samples);
        }
        results_dim.push(2);
        // Set the eigen value array and the eigen state array
        let eigenenergies: ArrayD<f64> = ArrayD::<f64>::zeros(IxDyn(&results_dim));
        // Add an extra dimension for the eigen states
        results_dim.push(2);
        let eigenstates: ArrayD<Complex64> = ArrayD::<Complex64>::zeros(IxDyn(&results_dim));
        return EigenstateResults { eigenstates: eigenstates, eigenenergies: eigenenergies };
    }
}

impl ExperimentResult for EigenstateResults {
    fn add_simulation_result(
        &mut self,
        sweep_parameter_indices: &Vec<usize>,
        simulation_result: &dyn SimulationResultGetter,
    ) -> () {

        // Get the hamiltonian at each sample
        let hamiltonians: Array3<Complex64> = simulation_result.get_hamiltonians().clone();

        // Array to store the eigen values and states at each sample
        let mut eigenstates: Array3<Complex64> = Array3::<Complex64>::zeros(hamiltonians.raw_dim());
        let mut eigenenergies: Array2<f64> = Array2::<f64>::zeros((hamiltonians.shape()[0], 2));
        
        // Loop over hamiltonians and find eigen states/energies
        for (i, hamiltonian) in hamiltonians.outer_iter().enumerate() {
            let (evals, evecs) = hamiltonian.eigh(UPLO::Lower).unwrap();
            eigenstates.index_axis_mut(Axis(0), i).assign(&evecs);
            eigenenergies.index_axis_mut(Axis(0), i).assign(&evals);
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

        if hamiltonians.shape()[0] == 1 {
            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
                SliceInfo::try_from(slice_info_vec.clone()).unwrap();

            self.eigenenergies
                .slice_mut(slice_info)
                .assign(&(&eigenenergies.index_axis(Axis(0), 0)));

            slice_info_vec.push(SliceInfoElem::Slice {
                start: 0,
                end: None,
                step: 1,
            });

            let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
                SliceInfo::try_from(slice_info_vec).unwrap();

            self.eigenstates
                .slice_mut(slice_info)
                .assign(&(&eigenstates.index_axis(Axis(0), 0)));

            return;
        }

        slice_info_vec.push(SliceInfoElem::Slice {
            start: 0,
            end: None,
            step: 1,
        });

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix2> =
            SliceInfo::try_from(slice_info_vec.clone()).unwrap();

        self.eigenenergies
            .slice_mut(slice_info)
            .assign(&eigenenergies);

        slice_info_vec.push(SliceInfoElem::Slice {
            start: 0,
            end: None,
            step: 1,
        });

        let slice_info: SliceInfo<Vec<SliceInfoElem>, IxDyn, Ix3> =
            SliceInfo::try_from(slice_info_vec).unwrap();

        self.eigenstates
            .slice_mut(slice_info)
            .assign(&eigenstates);
        return;
    }
    /// Save a given array of results to an HDF5 file. The results are N Dimensional where N should
    /// be the number of swept parameters. The size in each dimension corresponds to the number of
    /// values for the parameter across that axis. The file is saved under the given filename
    fn save(&self, group: &Group) -> Result<()> {
        // Make a builder and put the results data set into the file
        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.eigenstates).create("eigenstates")?;

        let builder = group.new_dataset_builder();
        let _ds = builder.with_data(&self.eigenenergies).create("eigenenergies")?;
        return Ok(());
    }
}
