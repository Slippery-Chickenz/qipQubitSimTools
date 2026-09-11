pub mod lab_frame;
pub mod pulse_frame;
pub mod rotating_frame;

pub use lab_frame::LabFrame;
pub use pulse_frame::PulseFrame;
pub use rotating_frame::RotatingFrame;

use crate::simulation::{Circuit, QubitArray};

use ndarray::Array2;
use num_complex::Complex64;

/// Reference frame to simulate in
#[derive(Debug, Clone)]
pub enum ReferenceFrame {
    Lab,
    Rotating,
    Pulse,
}

impl From<String> for ReferenceFrame {
    fn from(value: String) -> ReferenceFrame {
        match value.as_str() {
            "Lab" => ReferenceFrame::Lab,
            "Rotating" => ReferenceFrame::Rotating,
            "Pulse" => ReferenceFrame::Pulse,
            _ => panic!("Invalid frame!"),
        }
    }
}

pub trait Hamiltonian {
    fn get_matrix(
        circuit: &mut Circuit,
        qubit_array: &QubitArray,
        time: f64,
        time_index: usize,
    ) -> Array2<Complex64>;
}
