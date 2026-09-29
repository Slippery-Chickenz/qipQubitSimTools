use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::simulation::{Circuit, Hamiltonian, QubitArray, SimulationSettings, SimulationTimes};

use ndarray::{Array1, Array2, Array3, Array4};
use num_complex::Complex64;

pub trait SimulationMethod {
    type QubitStateType: Clone + Debug;
    type ResultType: SimulationResultSaver<QubitState = Self::QubitStateType>
        + SimulationResultGetter;
    fn evolve_state<T: Hamiltonian>(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        simulation_times: &SimulationTimes,
        qubit_state: Self::QubitStateType,
        _hamiltonian: PhantomData<T>,
        shot_num: usize,
        start_index: usize,
        end_index: usize,
    ) -> Self::QubitStateType;
    fn get_state(array: &Array1<Complex64>) -> Self::QubitStateType;
    fn get_num_times_per_step() -> usize;
}

pub trait SimulationResultSaver {
    type QubitState;
    fn new(simulation_times: Rc<SimulationTimes>, simulation_settings: &SimulationSettings)
    -> Self;
    fn save_state(&mut self, shot_num: usize, sample_num: usize, state: Self::QubitState) -> ();
    fn save_starting_state(&mut self, state: Self::QubitState) -> ();
    fn save_hamiltonian(
        &mut self,
        shot_num: usize,
        sample_num: usize,
        hamiltonian: Array2<Complex64>,
    ) -> ();
    fn save_starting_hamiltonian(&mut self, hamiltonian: Array2<Complex64>) -> ();
    fn save_larmors(&mut self, larmors: Array2<f64>) -> ();
}

pub trait SimulationResultGetter {
    fn get_probabilities(&self) -> Array2<f64>;
    fn get_state_probabilities(&self, state: &Array1<Complex64>) -> Array2<f64>;
    fn get_states(&self) -> Array3<Complex64>;
    fn get_duration(&self) -> f64;
    fn get_bloch_coords_cart(&self) -> (Array2<f64>, Array2<f64>, Array2<f64>);
    fn get_simulation_times(&self) -> &SimulationTimes;
    fn get_hamiltonians(&self) -> &Array4<Complex64>;
    fn take_larmors(&self) -> Array2<f64>;
}
