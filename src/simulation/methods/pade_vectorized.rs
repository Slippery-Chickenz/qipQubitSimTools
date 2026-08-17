use std::marker::PhantomData;

use crate::simulation::{
    Circuit, DensityMatrixVectorResult, Hamiltonian, QubitArray, SimulationMethod, SimulationTimes,
};

use ndarray::linalg::kron;
use ndarray::{Array1, Array2};
use ndarray_linalg::expm;
use num_complex::Complex64;

pub struct PadeVectorizedMethod {}

impl SimulationMethod for PadeVectorizedMethod {
    type QubitState = Array1<Complex64>;
    type ResultType = DensityMatrixVectorResult;
    fn evolve_state<T: Hamiltonian>(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        simulation_times: &SimulationTimes,
        mut qubit_state: Self::QubitState,
        _hamiltonian: PhantomData<T>,
        start_index: usize,
        end_index: usize,
    ) -> Self::QubitState {
        let dt: f64 = simulation_times.get_dt();
        for t_index in start_index..end_index {
            let time = simulation_times.get_iteration_time(t_index);
            let lindblad: Array2<Complex64> = PadeVectorizedMethod::get_lindblad_operator(
                T::get_matrix(circuit, qubit_array, time, t_index),
                qubit_array,
            );
            let evolution_operator: Array2<Complex64> = expm(&(lindblad * dt)).0;
            qubit_state = evolution_operator.dot(&qubit_state);
        }
        return qubit_state;
    }
    fn get_num_times_per_step() -> usize {
        return 4;
    }
    fn get_state(_array: &Array1<Complex64>) -> Array1<Complex64> {
        let mut density_matrix: Array1<Complex64> = Array1::<Complex64>::zeros(4);
        density_matrix[0] = Complex64::new(1., 0.);
        return density_matrix;
    }
}

impl PadeVectorizedMethod {
    /// Get the lindblad operator for the defined circuit at a specific time
    fn get_lindblad_operator(
        hamiltonian: Array2<Complex64>,
        qubit_array: &QubitArray,
    ) -> Array2<Complex64> {
        let identity: Array2<Complex64> = Array2::<Complex64>::eye(2);

        // let hamiltonian: Array2<Complex64> = PadeVectorizedMethod::get_hamiltonian_operator(
        //     circuit,
        //     qubit_array,
        //     reference_frame,
        //     time,
        //     time_index,
        // );

        let system_term: Array2<Complex64> = Complex64::new(0., -1.)
            * (kron(&identity, &hamiltonian) - kron(&hamiltonian.reversed_axes(), &identity));

        let mut jump_operator: Array2<Complex64> = Array2::<Complex64>::eye(2);
        jump_operator[[1, 1]] = Complex64::new(-1., 0.);

        let jump_operator_conj: Array2<Complex64> =
            jump_operator.clone().mapv(|x| x.conj()).reversed_axes();

        let dephasing_intensity: Complex64 = Complex64::new(qubit_array.get_decoherence(), 0.);

        let one: Array2<Complex64> =
            kron(&jump_operator_conj.clone().reversed_axes(), &jump_operator);
        let two: Array2<Complex64> = kron(&identity, &jump_operator_conj.dot(&jump_operator));
        let three: Array2<Complex64> = kron(
            &(jump_operator_conj.dot(&jump_operator).reversed_axes()),
            &identity,
        );

        let environment_term: Array2<Complex64> =
            0.5 * dephasing_intensity * (one * 2. - two - three);
        // * (jump_operator.dot(density_matrix).dot(&jump_operator_conj) * 2.
        //     - jump_operator_conj.dot(&jump_operator).dot(density_matrix)
        //     - density_matrix.dot(&jump_operator_conj).dot(&jump_operator));

        return system_term + environment_term;
    }
}
