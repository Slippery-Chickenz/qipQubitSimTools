pub mod lab_frame;
pub mod pulse_frame;
pub mod rotating_frame;

use crate::simulation::{Circuit, QubitArray};

use ndarray::{Array1, Array2};
use num_complex::Complex64;

/// Reference frame to simulate in
#[derive(Debug, Clone)]
pub enum ReferenceFrame {
    Lab,
    Rotating,
    Pulse,
}

pub trait Hamiltonian {
    fn get_matrix(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        time: f64,
        time_index: usize,
    ) -> Array2<Complex64>;
    fn get_vectorized(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        time: f64,
        time_index: usize,
    ) -> Array1<Complex64>;
}
