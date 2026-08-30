use std::rc::Rc;

use crate::simulation::{SimulationResultGetter, SimulationResultSaver, SimulationTimes};

use ndarray::{Array1, Array2, Array3, Axis};
use ndarray_linalg::trace::Trace;
use num_complex::Complex64;

pub struct DensityMatrixResult {
    /// Times for each sample from the simulation
    simulation_times: Rc<SimulationTimes>,
    /// Density matrix at each sample point
    density_matrices: Array3<Complex64>,
    /// Hamiltonian at each sample point
    hamiltonians: Array3<Complex64>,
}

impl SimulationResultSaver for DensityMatrixResult {
    type QubitState = Array2<Complex64>;
    /// Get a new SimulationResults object with a set set of simulation times and a starting
    /// density matrix
    fn new(simulation_times: Rc<SimulationTimes>, save_hamiltonian: bool) -> DensityMatrixResult {
        // Set the array of density matrices. The outer axis is the number of samples and the inner
        // axes are the 2x2 density matrices
        let density_matrices =
            Array3::<Complex64>::zeros([simulation_times.get_num_samples(), 2, 2]);
        let hamiltonians: Array3<Complex64>;
        if save_hamiltonian {
            hamiltonians = Array3::<Complex64>::zeros([simulation_times.get_num_samples(), 2, 2]);
        } else {
            hamiltonians = Array3::<Complex64>::zeros([1, 1, 1]);
        }
        return DensityMatrixResult {
            simulation_times: simulation_times,
            density_matrices: density_matrices,
            hamiltonians: hamiltonians,
        };
    }
    fn save_state(&mut self, sample_num: usize, state: Array2<Complex64>) -> () {
        // Set the next sample to the evolved state
        self.density_matrices
            .index_axis_mut(Axis(0), sample_num)
            .assign(&state);
        return;
    }
    fn save_hamiltonian(&mut self, sample_num: usize, hamiltonian: Array2<Complex64>) -> () {
        self.hamiltonians
            .index_axis_mut(Axis(0), sample_num)
            .assign(&hamiltonian);
        return;
    }
}

impl SimulationResultGetter for DensityMatrixResult {
    // Get the probability of every sample to be in the -z state
    fn get_probabilities(&self) -> Array1<f64> {
        return self.get_state_probabilities(&Array1::<Complex64>::from_vec(vec![
            Complex64::new(0., 0.),
            Complex64::new(1., 0.),
        ]));
    }
    /// Get the duration of the simulation
    fn get_duration(&self) -> f64 {
        return self.simulation_times.get_duration();
    }
    fn get_bloch_coords_cart(&self) -> (Array1<f64>, Array1<f64>, Array1<f64>) {
        // Coordinates to return
        let mut x_coords: Array1<f64> = Array1::<f64>::zeros(self.density_matrices.shape()[0]);
        let mut y_coords: Array1<f64> = Array1::<f64>::zeros(self.density_matrices.shape()[0]);
        let mut z_coords: Array1<f64> = Array1::<f64>::zeros(self.density_matrices.shape()[0]);

        // Loop over all samples and get/set the coordinates
        for i in 0..self.density_matrices.shape()[0] {
            let (x, y, z): (f64, f64, f64) = self.get_bloch_coord_cart(i);
            x_coords[i] = x;
            y_coords[i] = y;
            z_coords[i] = z;
        }
        return (x_coords, y_coords, z_coords);
    }
    fn get_simulation_times(&self) -> &SimulationTimes {
        return &self.simulation_times;
    }
    fn get_hamiltonians(&self) -> &Array3<Complex64> {
        return &self.hamiltonians;
    }
}

impl DensityMatrixResult {
    /// Get all the sampled density matrices
    pub fn get_density_matrices(&self) -> &Array3<Complex64> {
        return &self.density_matrices;
    }
    /// Get all the sampled density matrices
    pub fn get_density_matrix(&self, index: usize) -> Array2<Complex64> {
        return self
            .density_matrices
            .index_axis(Axis(0), index)
            .clone()
            .to_owned();
    }
    /// Get the simulation times for these results
    pub fn get_simulation_times(&self) -> Rc<SimulationTimes> {
        return Rc::clone(&self.simulation_times);
    }
    /// Get the probability that a certain sample number is in a given state
    pub fn get_probability(&self, sample_num: usize, state: &Array1<Complex64>) -> f64 {
        // Calculate the projection operator for the given state
        let projection_operator: Array2<Complex64> = state
            .to_shape([2, 1])
            .unwrap()
            .dot(&state.mapv(|x| x.conj()).to_shape([1, 2]).unwrap());
        // Return the trace of the density matrix dot producted with the projection operator
        return self
            .density_matrices
            .index_axis(Axis(0), sample_num)
            .dot(&projection_operator)
            .trace()
            .unwrap()
            .re;
    }
    /// Get the probability of the final sample to be in a given state
    pub fn get_final_state_probability(&self, state: &Array1<Complex64>) -> f64 {
        return self.get_probability(self.density_matrices.shape()[0] - 1, state);
    }
    /// Get the probability of the final sample to be in the -z state
    pub fn get_final_probability(&self) -> f64 {
        return self.get_final_state_probability(&Array1::<Complex64>::from_vec(vec![
            Complex64::new(0., 0.),
            Complex64::new(0., 1.),
        ]));
    }
    // Get the probability of every sample to be in a given state
    pub fn get_state_probabilities(&self, state: &Array1<Complex64>) -> Array1<f64> {
        // Make the array of probabilities to be the length of the number of samples
        let mut probabilities: Array1<f64> =
            Array1::<f64>::zeros([self.density_matrices.shape()[0]]);
        // Loop over all the number of samples and set the probabilities
        for i in 0..self.density_matrices.shape()[0] {
            probabilities[[i]] = self.get_probability(i, state);
        }
        return probabilities;
    }
    pub fn get_bloch_coord_cart(&self, sample_num: usize) -> (f64, f64, f64) {
        return (
            2. * self.density_matrices[[sample_num, 1, 0]].re,
            2. * self.density_matrices[[sample_num, 1, 0]].im,
            2. * self.density_matrices[[sample_num, 0, 0]].re - 1.,
        );
    }
}
