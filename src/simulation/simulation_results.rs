use std::rc::Rc;

use crate::simulation::SimulationTimes;

use ndarray::{Array1, Array2, Array3, Array4, Axis, s};
use ndarray_linalg::trace::Trace;
use num_complex::Complex64;

/// Results from a simulation. Holds the times and the density matrices at each sample time
pub struct SimulationResults {
    /// Times for each sample from the simulation
    simulation_times: Rc<SimulationTimes>,
    /// Density matrix at each sample point
    density_matrices: Array3<Complex64>,
}

impl SimulationResults {
    /// Get a new SimulationResults object with a set set of simulation times and a starting
    /// density matrix
    pub fn new(
        simulation_times: Rc<SimulationTimes>,
        starting_density_matrx: &Array2<Complex64>,
    ) -> SimulationResults {
        // Set the array of density matrices. The outer axis is the number of samples and the inner
        // axes are the 2x2 density matrices
        let mut density_matrices =
            Array3::<Complex64>::zeros([simulation_times.get_num_samples(), 2, 2]);
        // Assign the first matrix to the starting density matrix
        density_matrices
            .index_axis_mut(Axis(0), 0)
            .assign(starting_density_matrx);
        return SimulationResults {
            simulation_times: simulation_times,
            density_matrices: density_matrices,
        };
    }
    pub fn save_state(&mut self, sample_num: usize, state: Array2<Complex64>) -> () {
        // Set the next sample to the evolved state
        self.density_matrices
            .index_axis_mut(Axis(0), sample_num + 1)
            .assign(&state);
        return;
    }
    /// Given a set of evolution operators take a sample number and evolve it and save it as the
    /// next sample.
    pub fn evolve_state(
        &mut self,
        sample_num: usize,
        evolution_operators: Array4<Complex64>,
        channel_coefficients: &Array2<Complex64>,
    ) -> () {
        // Copy the last state to evolve as the next sampled state
        let mut next_sample: Array2<Complex64> = self
            .density_matrices
            .index_axis(Axis(0), sample_num)
            .clone()
            .to_owned();
        // Loop over the evolution operators and dot them on either side of the density matrix to evolve
        for iter in evolution_operators.outer_iter() {
            next_sample = iter
                .index_axis(Axis(0), 0)
                .dot(&next_sample.dot(&iter.index_axis(Axis(0), 1)));
            next_sample *= channel_coefficients;
        }
        // Set the next sample to the evolved state
        self.density_matrices
            .index_axis_mut(Axis(0), sample_num + 1)
            .assign(&next_sample);
        return;
    }
    pub fn evolve_state_hamiltonian(
        &mut self,
        sample_num: usize,
        hamiltonians: Array3<Complex64>,
        dt: f64,
    ) -> () {
        // Copy the last state to evolve as the next sampled state
        let mut next_sample: Array2<Complex64> = self
            .density_matrices
            .index_axis(Axis(0), sample_num)
            .clone()
            .to_owned();
        let mut prev_state: Array2<Complex64> = next_sample.clone();
        // Loop over the evolution operators and dot them on either side of the density matrix to evolve
        for iter in hamiltonians.outer_iter() {
            next_sample +=
                &(Complex64::new(0., -dt) * (iter.dot(&prev_state) - prev_state.dot(&iter)));
            prev_state = next_sample.clone();
        }
        // Set the next sample to the evolved state
        self.density_matrices
            .index_axis_mut(Axis(0), sample_num + 1)
            .assign(&next_sample);

        return;
    }
    /// Get all the sampled density matrices
    pub fn get_density_matrices(&self) -> &Array3<Complex64> {
        return &self.density_matrices;
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
    // Get the probability of every sample to be in the -z state
    pub fn get_probabilities(&self) -> Array1<f64> {
        return self.get_state_probabilities(&Array1::<Complex64>::from_vec(vec![
            Complex64::new(0., 0.),
            Complex64::new(1., 0.),
        ]));
    }
    pub fn get_bloch_coords_cart(&self) -> (Array1<f64>, Array1<f64>, Array1<f64>) {
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
    pub fn get_bloch_coord_cart(&self, sample_num: usize) -> (f64, f64, f64) {
        return (
            2. * self.density_matrices[[sample_num, 1, 0]].re,
            2. * self.density_matrices[[sample_num, 1, 0]].im,
            2. * self.density_matrices[[sample_num, 0, 0]].re - 1.,
        );
    }
    pub fn remove_starting_sample(&mut self) -> () {
        self.density_matrices = self.density_matrices.clone().slice_move(s![1.., .., ..]);
        return;
    }
}
