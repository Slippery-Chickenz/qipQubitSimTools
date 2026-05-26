use std::rc::Rc;
use std::fmt::Debug;

use crate::simulation::{Circuit, QubitArray, SimulationTimes};

use ndarray::Array1;
use num_complex::Complex64;

pub trait SimulationMethod {
    type QubitState: Clone + Debug;
    type ResultType: SimulationResultSaver<QubitState = Self::QubitState> + SimulationResultGetter;
    fn evolve_state(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        simulation_times: &SimulationTimes,
        qubit_state: Self::QubitState,
        start_index: usize,
        end_index: usize,
    ) -> Self::QubitState;
    fn get_state(array: &Array1<Complex64>) -> Self::QubitState;
}

pub trait SimulationResultSaver {
    type QubitState;
    fn new(simulation_times: Rc<SimulationTimes>) -> Self;
    fn save_state(&mut self, sample_num: usize, state: Self::QubitState) -> ();
}

pub trait SimulationResultGetter {
    fn get_probabilities(&self) -> Array1<f64>;
    fn get_duration(&self) -> f64;
    fn get_bloch_coords_cart(&self) -> (Array1<f64>, Array1<f64>, Array1<f64>);
}
