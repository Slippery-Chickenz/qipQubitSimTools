use crate::simulation::{
    Circuit, QubitArray, QubitStateResult, ReferenceFrame, SimulationMethod, SimulationTimes,
};
use std::f64::consts::PI;

use ndarray::{Array1, Array2, arr2};
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
        reference_frame: &ReferenceFrame,
        start_index: usize,
        end_index: usize,
    ) -> Self::QubitState {
        let dt: f64 = simulation_times.get_dt();
        for t_index in start_index..end_index {
            let time = simulation_times.get_iteration_time(t_index);
            let hamiltonain: Array2<Complex64> = PadeStateMethod::get_hamiltonian_operator(
                circuit,
                qubit_array,
                reference_frame,
                time,
                t_index,
            );
            let evolution_operator: Array2<Complex64> =
                expm(&(Complex64::new(0., -1.) * hamiltonain * dt)).0;
            qubit_state = evolution_operator.dot(&qubit_state);
        }
        return qubit_state;
    }
    fn get_num_times_per_step() -> usize {
        return 4;
    }
    fn get_state(_array: &Array1<Complex64>) -> Array1<Complex64> {
        let mut density_matrix: Array1<Complex64> = Array1::<Complex64>::zeros(2);
        density_matrix[0] = Complex64::new(1., 0.);
        return density_matrix;
    }
}

impl PadeStateMethod {
    /// Get the Hamiltonian operator for the defined circuit at a specific time
    fn get_hamiltonian_operator(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        reference_frame: &ReferenceFrame,
        time: f64,
        time_index: usize,
    ) -> Array2<Complex64> {
        let larmor_frequency: f64 = qubit_array.get_larmor_frequency(time_index);
        let guess_frequency: f64 = qubit_array.get_guess_larmor();
        let amplitude: f64 = circuit.get_amplitude(time);
        let integrated_frequency: f64 = circuit.get_integrated_frequency(time);
        let phase: f64 = circuit.get_phase(time);

        match reference_frame {
            ReferenceFrame::Lab => {
                return PadeStateMethod::lab_frame_hamiltonian(
                    larmor_frequency,
                    guess_frequency,
                    amplitude,
                    integrated_frequency,
                    phase,
                );
            }
            ReferenceFrame::Rotating => {
                return PadeStateMethod::rotating_frame_hamiltonian(
                    larmor_frequency,
                    guess_frequency,
                    amplitude,
                    integrated_frequency,
                    phase,
                );
            }
            ReferenceFrame::Pulse => {
                return PadeStateMethod::lab_frame_hamiltonian(
                    larmor_frequency,
                    guess_frequency,
                    amplitude,
                    integrated_frequency,
                    phase,
                );
            }
        }
        // return circuit.get_hamiltonian_operator(time)
        //     + qubit_array.get_detuning_hamiltonian(time_index);
    }

    fn lab_frame_hamiltonian(
        larmor_frequency: f64,
        guess_frequency: f64,
        amplitude: f64,
        integrated_frequency: f64,
        phase: f64,
    ) -> Array2<Complex64> {
        return arr2(&[
            [
                PI * Complex64::new(larmor_frequency - guess_frequency, 0.),
                amplitude
                    * PI
                    * 0.5
                    * Complex64::new(
                        (2. * PI * integrated_frequency + phase).cos(),
                        -(2. * PI * integrated_frequency + phase).sin(),
                    ),
            ],
            [
                amplitude
                    * PI
                    * 0.5
                    * Complex64::new(
                        (2. * PI * integrated_frequency + phase).cos(),
                        (2. * PI * integrated_frequency + phase).sin(),
                    ),
                PI * Complex64::new(guess_frequency - larmor_frequency, 0.),
            ],
        ]);
    }
    fn rotating_frame_hamiltonian(
        larmor_frequency: f64,
        guess_frequency: f64,
        amplitude: f64,
        integrated_frequency: f64,
        phase: f64,
    ) -> Array2<Complex64> {
        return arr2(&[
            [
                PI * Complex64::new(larmor_frequency - guess_frequency, 0.),
                amplitude
                    * PI
                    * 0.5
                    * Complex64::new(
                        (2. * PI * integrated_frequency + phase).cos(),
                        -(2. * PI * integrated_frequency + phase).sin(),
                    ),
            ],
            [
                amplitude
                    * PI
                    * 0.5
                    * Complex64::new(
                        (2. * PI * integrated_frequency + phase).cos(),
                        (2. * PI * integrated_frequency + phase).sin(),
                    ),
                PI * Complex64::new(guess_frequency - larmor_frequency, 0.),
            ],
        ]);
    }
}
