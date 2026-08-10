use crate::simulation::{Circuit, Hamiltonian, QubitArray};
use std::f64::consts::PI;

use ndarray::{Array1, Array2, arr1, arr2};
use num_complex::Complex64;

pub struct PulseFrame {}

impl Hamiltonian for PulseFrame {
    fn get_matrix(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        time: f64,
        time_index: usize,
    ) -> Array2<Complex64> {
        let larmor_frequency: f64 = qubit_array.get_larmor_frequency(time_index);
        let guess_frequency: f64 = qubit_array.get_guess_larmor();
        let amplitude: f64 = circuit.get_amplitude(time);
        let pulse_frequency: f64 = circuit.get_frequency(time);
        return arr2(&[
            [
                PI * Complex64::new(larmor_frequency - guess_frequency - pulse_frequency, 0.),
                Complex64::new(amplitude * PI * 0.5, 0.),
            ],
            [
                Complex64::new(amplitude * PI * 0.5, 0.),
                PI * Complex64::new(guess_frequency - larmor_frequency + pulse_frequency, 0.),
            ],
        ]);
    }
    fn get_vectorized(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        time: f64,
        time_index: usize,
    ) -> Array1<Complex64> {
        let larmor_frequency: f64 = qubit_array.get_larmor_frequency(time_index);
        let guess_frequency: f64 = qubit_array.get_guess_larmor();
        let amplitude: f64 = circuit.get_amplitude(time);
        let pulse_frequency: f64 = circuit.get_frequency(time);
        return arr1(&[
            PI * Complex64::new(larmor_frequency - guess_frequency - pulse_frequency, 0.),
            Complex64::new(amplitude * PI * 0.5, 0.),
            Complex64::new(amplitude * PI * 0.5, 0.),
            PI * Complex64::new(guess_frequency - larmor_frequency + pulse_frequency, 0.),
        ]);
    }
}
