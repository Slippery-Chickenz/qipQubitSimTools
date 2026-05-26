use crate::simulation::{
    Circuit, QubitStateResult, QubitArray, SimulationMethod, SimulationTimes,
};

use ndarray::{Array1, Array2};
use ndarray_linalg::expm;
use num_complex::Complex64;

pub struct PadeStateMethod {}

impl SimulationMethod for PadeStateMethod {
    type QubitState = Array1<Complex64>;
    type ResultType = QubitStateResult;
    fn evolve_state(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        simulation_times: &SimulationTimes,
        mut qubit_state: Self::QubitState,
        start_index: usize,
        end_index: usize,
    ) -> Self::QubitState {
        let dt: f64 = simulation_times.get_dt();
        for t_index in start_index..end_index {
            let time = simulation_times.get_iteration_time(t_index);
            let hamiltonain: Array2<Complex64> = PadeStateMethod::get_hamiltonian_operator(circuit, qubit_array, time);
            let evolution_operator: Array2<Complex64> = expm(&(Complex64::new(0., -1.) * hamiltonain * dt)).0;
            qubit_state = evolution_operator.dot(&qubit_state);
        }
        return qubit_state;
    }
    fn get_state(_array: &Array1<Complex64>) -> Array1<Complex64> {
        let mut density_matrix: Array1<Complex64> = Array1::<Complex64>::zeros(2);
        density_matrix[0] = Complex64::new(1., 0.);
        return density_matrix;
    }
}

impl PadeStateMethod {
    /// Get the Hamiltonian operator for teh defined circuit at a specific time
    fn get_hamiltonian_operator (
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        time: f64,
    ) -> Array2<Complex64> {
        return circuit.get_hamiltonian_operator(time) + qubit_array.get_detuning_hamiltonian();
    }
}
