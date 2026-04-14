use std::f64::consts::PI;
use std::rc::Rc;

use crate::simulation_times::{SimulationTimes, UninitializedTimesError};

use ndarray::linalg::kron;
use ndarray::{Array2, Array3};
use num_complex::Complex64;

#[derive(Clone)]
pub struct QubitArray {
    num_qubts: u32,
    density_matrix: Array2<Complex64>,
    larmor: f64,
    guess_larmor: f64,
    simulation_times: Option<Rc<SimulationTimes>>,
}

impl QubitArray {
    pub fn new(num_qubits: u32, larmor: f64, guess_larmor: f64) -> QubitArray {
        let mut density_matrix: Array2<Complex64> = Array2::<Complex64>::zeros((2, 2));
        density_matrix[[0, 0]] = Complex64::new(1., 0.);

        let temp_matrix: Array2<Complex64> = density_matrix.clone();

        for _i in 0..num_qubits - 1 {
            density_matrix = kron(&density_matrix, &temp_matrix);
        }

        return QubitArray {
            num_qubts: num_qubits,
            density_matrix: density_matrix,
            larmor: larmor,
            guess_larmor: guess_larmor,
            simulation_times: None,
        };
    }
    pub fn set_simulation_times(&mut self, simulation_times: Rc<SimulationTimes>) -> () {
        self.simulation_times = Some(simulation_times);
        return;
    }
    pub fn get_density_matrix(&self) -> &Array2<Complex64> {
        return &self.density_matrix;
    }
    pub fn get_num_qubits(&self) -> u32 {
        return self.num_qubts;
    }
    pub fn get_detuning_hamiltonians(&self) -> Array3<Complex64> {
        // Detuning between guess and qubit
        let detuning: f64 = 2. * PI * (self.guess_larmor - self.larmor);

        let detuning_hamiltonian: Array3<Complex64> = Array3::<Complex64>::from_shape_fn(
            (
                self.simulation_times
                    .as_ref()
                    .ok_or(UninitializedTimesError)
                    .unwrap()
                    .get_num_iterations_per_sample(),
                2,
                2,
            ),
            |(_i, j, k)| {
                Complex64::new(
                    f64::from(-(i32::try_from(j + k).unwrap() - 1)) * detuning,
                    0.,
                )
            },
        );
        return detuning_hamiltonian;
    }
}
