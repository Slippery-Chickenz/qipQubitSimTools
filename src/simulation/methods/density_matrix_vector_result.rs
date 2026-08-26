use std::rc::Rc;

use crate::simulation::{SimulationResultGetter, SimulationResultSaver, SimulationTimes};

use ndarray::{Array1, Array2, Axis};
use ndarray_linalg::trace::Trace;
use num_complex::Complex64;

pub struct DensityMatrixVectorResult {
    /// Times for each sample from the simulation
    simulation_times: Rc<SimulationTimes>,
    /// Density matrix at each sample point
    density_matrices: Array2<Complex64>,
}

impl SimulationResultSaver for DensityMatrixVectorResult {
    type QubitState = Array1<Complex64>;
    /// Get a new SimulationResults object with a set set of simulation times and a starting
    /// density matrix
    fn new(simulation_times: Rc<SimulationTimes>) -> DensityMatrixVectorResult {
        // Set the array of density matrices. The outer axis is the number of samples and the inner
        // axes are the 2x2 density matrices
        let density_matrices = Array2::<Complex64>::zeros([simulation_times.get_num_samples(), 4]);
        return DensityMatrixVectorResult {
            simulation_times: simulation_times,
            density_matrices: density_matrices,
        };
    }
    fn save_state(&mut self, sample_num: usize, state: Array1<Complex64>) -> () {
        // Set the next sample to the evolved state
        self.density_matrices
            .index_axis_mut(Axis(0), sample_num)
            .assign(&state);
        return;
    }
}

impl SimulationResultGetter for DensityMatrixVectorResult {
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
}

impl DensityMatrixVectorResult {
    /// Get all the sampled density matrices
    pub fn get_density_matrices(&self) -> &Array2<Complex64> {
        return &self.density_matrices;
    }
    /// Get all the sampled density matrices
    pub fn get_density_matrix(&self, index: usize) -> Array1<Complex64> {
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

        let mut temp_density_matrix: Array2<Complex64> = Array2::<Complex64>::zeros([2, 2]);
        temp_density_matrix[[0, 0]] = self.density_matrices[[sample_num, 0]];
        temp_density_matrix[[1, 0]] = self.density_matrices[[sample_num, 1]];
        temp_density_matrix[[0, 1]] = self.density_matrices[[sample_num, 2]];
        temp_density_matrix[[1, 1]] = self.density_matrices[[sample_num, 3]];
        // Return the trace of the density matrix dot producted with the projection operator
        return temp_density_matrix
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
            2. * self.density_matrices[[sample_num, 1]].re,
            2. * self.density_matrices[[sample_num, 1]].im,
            2. * self.density_matrices[[sample_num, 0]].re - 1.,
        );
    }
}
