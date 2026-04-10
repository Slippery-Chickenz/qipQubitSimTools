use std::rc::Rc;
use std::f64::consts::PI;

use crate::simulation_times::SimulationTimes;

use num_complex::Complex64;
use ndarray::{ Array1, Array2, Array3, Array4, Axis };
use ndarray::linalg::kron;
use ndarray_linalg::trace::Trace;

#[derive(Clone)]
pub struct QubitArray {
    num_qubts: u32,
    density_matrices: Vec<Array2<Complex64>>,
    larmor: f64,
    simulation_times: Rc<SimulationTimes>,
}

impl QubitArray {
    pub fn new(num_qubits: u32, larmor: f64) -> QubitArray {

        let mut density_matrix: Array2<Complex64> = Array2::<Complex64>::zeros((2, 2));
        density_matrix[[0, 0]] = Complex64::new(1., 0.);

        let temp_matrix: Array2<Complex64> = density_matrix.clone();

        for _i in 0..num_qubits-1 {
            density_matrix = kron(&density_matrix, &temp_matrix);
        }

        return QubitArray { num_qubts: num_qubits, 
                            density_matrices: vec![density_matrix], 
                            larmor: larmor, 
                            simulation_times: Rc::new(SimulationTimes::new(10., 3, 2)) };
    }
    pub fn get_density_matrices(&self) -> &Vec<Array2<Complex64>> {
        return &self.density_matrices;
    }
    pub fn get_probability(&self, sample_num: usize, state: Array1<Complex64>) -> f64 {
        let projection_operator: Array2<Complex64> = state.to_shape([2, 1]).unwrap().dot(&state.mapv(|x| x.conj()).to_shape([1, 2]).unwrap());
        return self.density_matrices[sample_num].dot(&projection_operator).trace().unwrap().re;
    }
    pub fn get_bloch_coords_cart(&self, sample_num: usize) -> (f64, f64, f64) {
        return (2. * self.density_matrices[sample_num][[1, 0]].re, 
                2. * self.density_matrices[sample_num][[1, 0]].im,
                2. * self.density_matrices[sample_num][[0, 0]].re - 1.);
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

        let detuning_hamiltonian: Array3<Complex64> = Array3::<Complex64>::from_shape_fn((self.simulation_times.get_num_iterations_per_sample(), 2, 2), 
                                                                                         |(_i, j, k)| Complex64::new(f64::from(-(i32::try_from(j + k).unwrap() - 1)) * detuning, 0. ));
        return detuning_hamiltonian;
    }
    pub fn evolve_state(&mut self, evolution_operators: Array4<Complex64>) -> () {

        // Copy the last state to evolve as the next sampled state
        let mut next_sample: Array2<Complex64> = self.density_matrices.last().cloned().unwrap();
        
        for iter in evolution_operators.outer_iter() {
            next_sample = iter.index_axis(Axis(0), 0).dot(&next_sample.dot(&iter.index_axis(Axis(0), 1)));
        }

        self.density_matrices.push(next_sample);
        return;
    }
}
