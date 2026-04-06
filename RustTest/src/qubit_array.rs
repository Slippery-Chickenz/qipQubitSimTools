use std::rc::Rc;
use std::f64::consts::PI;

use crate::simulation_times::SimulationTimes;

use num_complex::Complex64;
use ndarray::{ Array2, Array3 };
use ndarray::linalg::kron;

#[derive(Clone)]
pub struct QubitArray {
    num_qubts: u32,
    _density_matrix: Array2<f64>,
    larmor: f64,
    simulation_times: Rc<SimulationTimes>,
}

impl QubitArray {
    pub fn new(num_qubits: u32, larmor: f64) -> QubitArray {

        let mut density_matrix: Array2<f64> = Array2::<f64>::zeros((2, 2));
        density_matrix[[0, 0]] = 1.;

        let temp_matrix: Array2<f64> = density_matrix.clone();

        for _i in 0..num_qubits {
            density_matrix = kron(&density_matrix, &temp_matrix);
        }

        return QubitArray { num_qubts: num_qubits, _density_matrix: density_matrix, larmor: larmor, simulation_times: Rc::new(SimulationTimes::new(0., 0, 0)) };
    }
    pub fn set_simulation_times(&mut self, simulation_times: Rc<SimulationTimes>) -> () {
        self.simulation_times = simulation_times;
        return;
    }
    pub fn get_num_qubits(&self) -> u32 {
        return self.num_qubts;
    }
    pub fn get_detuning_hamiltonians(&self, guess_larmor: f64) -> Array3<Complex64> {

        // Detuning between guess and qubit
        let detuning: f64 = 2. * PI * (guess_larmor - self.larmor);

        // let mut detuning_hamiltonian: Array3<Complex64> = Array3::<Complex64>::zeros([self.simulation_times.get_num_iterations_per_sample(), 2, 2]);
        let detuning_hamiltonian: Array3<Complex64> = Array3::<Complex64>::from_shape_fn((self.simulation_times.get_num_iterations_per_sample(), 2, 2), 
                                                                                             |(_i, j, k)| Complex64 {re: f64::from(-(i32::try_from(j + k).unwrap() - 1)) * detuning, im: 0. } );
        return detuning_hamiltonian;
    }
}
