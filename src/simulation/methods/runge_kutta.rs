use crate::simulation::{
    Circuit, DensityMatrixResult, QubitArray, SimulationMethod, SimulationTimes,
};

use ndarray::{Array1, Array2};
use num_complex::Complex64;

pub struct RKMethod {}

impl SimulationMethod for RKMethod {
    type QubitState = Array2<Complex64>;
    type ResultType = DensityMatrixResult;
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

            // Four coefficients for Runge Kutta
            let k_1: Array2<Complex64> =
                RKMethod::get_lindblad_operator(circuit, qubit_array, time, t_index, &qubit_state);
            let k_2: Array2<Complex64> = RKMethod::get_lindblad_operator(
                circuit,
                qubit_array,
                time + (dt / 2.),
                t_index + 1,
                &(&qubit_state + (&k_1 * (dt / 2.))),
            );
            let k_3: Array2<Complex64> = RKMethod::get_lindblad_operator(
                circuit,
                qubit_array,
                time + (dt / 2.),
                t_index + 2,
                &(&qubit_state + (&k_2 * (dt / 2.))),
            );
            let k_4: Array2<Complex64> = RKMethod::get_lindblad_operator(
                circuit,
                qubit_array,
                time + dt,
                t_index + 3,
                &(&qubit_state + (&k_3 * dt)),
            );

            let next_state = qubit_state + (k_1 + (k_2 * 2.) + (k_3 * 2.) + k_4) * (dt / 6.);
            qubit_state = next_state;
        }
        return qubit_state;
    }
    fn get_num_times_per_step() -> usize {
        return 4;
    }
    fn get_state(_array: &Array1<Complex64>) -> Array2<Complex64> {
        let mut density_matrix: Array2<Complex64> = Array2::<Complex64>::zeros((2, 2));
        density_matrix[[0, 0]] = Complex64::new(1., 0.);
        return density_matrix;
    }
}

impl RKMethod {
    /// Get the lindblad operator for the defined circuit at a specific time
    fn get_lindblad_operator(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        time: f64,
        time_index: usize,
        density_matrix: &Array2<Complex64>,
    ) -> Array2<Complex64> {
        let hamiltonian: Array2<Complex64> = circuit.get_hamiltonian_operator(time)
            + qubit_array.get_detuning_hamiltonian(time_index);

        let system_term: Array2<Complex64> = Complex64::new(0., -1.)
            * (hamiltonian.dot(density_matrix) - density_matrix.dot(&hamiltonian));

        let mut jump_operator: Array2<Complex64> = Array2::<Complex64>::eye(2);
        jump_operator[[1, 1]] = Complex64::new(-1., 0.);

        let jump_operator_conj: Array2<Complex64> =
            jump_operator.clone().mapv(|x| x.conj()).reversed_axes();

        let dephasing_intensity: Complex64 = Complex64::new(qubit_array.get_decoherence(), 0.);

        let environment_term: Array2<Complex64> = 0.5
            * dephasing_intensity
            * (jump_operator.dot(density_matrix).dot(&jump_operator_conj) * 2.
                - jump_operator_conj.dot(&jump_operator).dot(density_matrix)
                - density_matrix.dot(&jump_operator_conj).dot(&jump_operator));

        return system_term + environment_term;
    }
}
