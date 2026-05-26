use std::rc::Rc;

use crate::simulation::{SimulationResultGetter, SimulationResultSaver, SimulationTimes};

use ndarray::{Array1, Array2, Axis};
use num_complex::Complex64;

pub struct QubitStateResult {
    /// Times for each sample from the simulation
    simulation_times: Rc<SimulationTimes>,
    /// Qubit state at each sample point in the z-basis
    states: Array2<Complex64>,
}

impl SimulationResultSaver for QubitStateResult {
    type QubitState = Array1<Complex64>;
    /// Get a new result object with a set set of simulation times and a starting
    /// state
    fn new(simulation_times: Rc<SimulationTimes>) -> QubitStateResult {
        // Set the array of qubit states. The outer axis is the number of samples and the inner
        // axes are the states in the z basis
        let qubit_states = Array2::<Complex64>::zeros([simulation_times.get_num_samples(), 2]);
        return QubitStateResult {
            simulation_times: simulation_times,
            states: qubit_states,
        };
    }
    fn save_state(&mut self, sample_num: usize, state: Array1<Complex64>) -> () {
        // Set the next sample to the evolved state
        self.states
            .index_axis_mut(Axis(0), sample_num)
            .assign(&state);
        return;
    }
}

impl SimulationResultGetter for QubitStateResult {
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
        let mut x_coords: Array1<f64> = Array1::<f64>::zeros(self.states.shape()[0]);
        let mut y_coords: Array1<f64> = Array1::<f64>::zeros(self.states.shape()[0]);
        let mut z_coords: Array1<f64> = Array1::<f64>::zeros(self.states.shape()[0]);

        // Loop over all samples and get/set the coordinates
        for i in 0..self.states.shape()[0] {
            let (x, y, z): (f64, f64, f64) = self.get_bloch_coord_cart(i);
            x_coords[i] = x;
            y_coords[i] = y;
            z_coords[i] = z;
        }
        return (x_coords, y_coords, z_coords);
    }
}

impl QubitStateResult {
    /// Get all the sampled states
    pub fn get_all_states(&self) -> &Array2<Complex64> {
        return &self.states;
    }
    /// Get a specific state sample
    pub fn get_state(&self, index: usize) -> Array1<Complex64> {
        return self
            .states
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
        let inner_product: Complex64 = state.mapv(|x| x.conj()).dot(&self.states.index_axis(Axis(0), sample_num));
        return (inner_product.conj() * inner_product).re;
    }
    /// Get the probability of the final sample to be in a given state
    pub fn get_final_state_probability(&self, state: &Array1<Complex64>) -> f64 {
        return self.get_probability(self.states.shape()[0] - 1, state);
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
            Array1::<f64>::zeros([self.states.shape()[0]]);
        // Loop over all the number of samples and set the probabilities
        for i in 0..self.states.shape()[0] {
            probabilities[[i]] = self.get_probability(i, state);
        }
        return probabilities;
    }
    pub fn get_bloch_coord_cart(&self, sample_num: usize) -> (f64, f64, f64) {

        // Sample state to find the bloch coord for
        let sample_state: Array1<Complex64> = self.states.index_axis(Axis(0), sample_num).to_owned();

        // Calculate the density matrix for the state
        let density_matrix: Array2<Complex64> = sample_state
            .to_shape([2, 1])
            .unwrap()
            .dot(&sample_state.mapv(|x| x.conj()).to_shape([1, 2]).unwrap());
        return (
            2. * density_matrix[[0, 1]].re,
            2. * density_matrix[[0, 1]].im,
            2. * density_matrix[[0, 0]].re - 1.,
        );
    }
}
