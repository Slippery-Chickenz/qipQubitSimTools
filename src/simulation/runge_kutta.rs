use crate::simulation::{Circuit, QubitArray, SimulationTimes};

use ndarray::{Array1, Array2};
use ndarray::linalg::kron;
use num_complex::Complex64;

#[inline]
pub fn runge_kutta_evolve(
    circuit: &mut Circuit,
    qubit_array: &QubitArray,
    simulation_times: &SimulationTimes,
    mut qubit_state: Array1<Complex64>, 
    start_index: usize, 
    end_index: usize
    ) -> Array1<Complex64> {

    let dt: f64 = simulation_times.get_dt();
    for t_index in start_index..end_index {
        let time = simulation_times.get_iteration_time(t_index);

        // Four coefficients for Runge Kutta
        let k_1: Array1<Complex64> = get_lindblad_operator(
            circuit,
            qubit_array,
            time,
        ).dot(&qubit_state);
        let k_2: Array1<Complex64> = get_lindblad_operator(
            circuit,
            qubit_array,
            time + (dt / 2.),
        ).dot(&(&qubit_state + (&k_1 * dt)));
        let k_3: Array1<Complex64> = get_lindblad_operator(
            circuit,
            qubit_array,
            time + (dt / 2.),
        ).dot(&(&qubit_state + (&k_2 * dt)));
        let k_4: Array1<Complex64> = get_lindblad_operator(
            circuit,
            qubit_array,
            time + dt,
        ).dot(&(&qubit_state + (&k_3 * dt)));

        let next_state = qubit_state + (k_1 + (k_2 * 2.) + (k_3 * 2.) + k_4) * (dt / 6.);
        qubit_state = next_state;
    }
    return qubit_state;
}

/// Get the lindblad operator for the defined circuit at a specific time
#[inline]
fn get_lindblad_operator(
    circuit: &mut Circuit,
    qubit_array: &QubitArray,
    time: f64,
) -> Array2<Complex64> {

    let identity: Array2<Complex64> = Array2::<Complex64>::eye(2);

    let hamiltonian: Array2<Complex64> =
        circuit.get_hamiltonian_operator(time) + qubit_array.get_detuning_hamiltonian();

    let system_term: Array2<Complex64> = Complex64::new(0., -1.)
        * (kron(&identity, &hamiltonian) - kron(&hamiltonian.reversed_axes(), &identity));

    let mut jump_operator: Array2<Complex64> = Array2::<Complex64>::eye(2);
    jump_operator[[1, 1]] = Complex64::new(-1., 0.);

    let jump_operator_conj: Array2<Complex64> =
        jump_operator.clone().mapv(|x| x.conj()).reversed_axes();

    let dephasing_intensity: Complex64 = Complex64::new(qubit_array.get_decoherence(), 0.);

    let one: Array2<Complex64> = kron(&jump_operator_conj.clone().reversed_axes(), & jump_operator);
    let two: Array2<Complex64> = kron(&identity, &jump_operator_conj.dot(&jump_operator));
    let three: Array2<Complex64> = kron(&(jump_operator_conj.dot(&jump_operator).reversed_axes()), &identity);

    let environment_term: Array2<Complex64> = 0.5
        * dephasing_intensity
        * (one * 2.
            - two
            - three);
        // * (jump_operator.dot(density_matrix).dot(&jump_operator_conj) * 2.
        //     - jump_operator_conj.dot(&jump_operator).dot(density_matrix)
        //     - density_matrix.dot(&jump_operator_conj).dot(&jump_operator));

    return system_term + environment_term;
}
