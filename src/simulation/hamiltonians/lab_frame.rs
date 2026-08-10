use crate::simulation::{Circuit, Hamiltonian, QubitArray};
use std::f64::consts::PI;

use ndarray::{Array1, Array2, arr1, arr2};
use num_complex::Complex64;

pub struct LabFrame {}

impl Hamiltonian for LabFrame {
    fn get_matrix(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        time: f64,
        time_index: usize,
    ) -> Array2<Complex64> {
        let larmor_frequency: f64 = qubit_array.get_larmor_frequency(time_index);
        let guess_frequency: f64 = qubit_array.get_guess_larmor();
        let amplitude: f64 = circuit.get_amplitude(time);
        let integrated_frequency: f64 = circuit.get_integrated_frequency(time);
        let phase: f64 = circuit.get_phase(time);
        return arr2(&[
            [
                PI * Complex64::new(larmor_frequency, 0.),
                amplitude
                    * PI
                    * Complex64::new(
                        (2. * PI * ((guess_frequency * time) + integrated_frequency + phase)).cos(),
                        0.,
                    ),
            ],
            [
                amplitude
                    * PI
                    * Complex64::new(
                        (2. * PI * ((guess_frequency * time) + integrated_frequency + phase)).cos(),
                        0.,
                    ),
                PI * Complex64::new(-larmor_frequency, 0.),
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
        let integrated_frequency: f64 = circuit.get_integrated_frequency(time);
        let phase: f64 = circuit.get_phase(time);
        return arr1(&[
            PI * Complex64::new(larmor_frequency, 0.),
            amplitude
                * PI
                * Complex64::new(
                    (2. * PI * ((guess_frequency * time) + integrated_frequency + phase)).cos(),
                    0.,
                ),
            amplitude
                * PI
                * Complex64::new(
                    (2. * PI * ((guess_frequency * time) + integrated_frequency + phase)).cos(),
                    0.,
                ),
            PI * Complex64::new(-larmor_frequency, 0.),
        ]);
    }
}
