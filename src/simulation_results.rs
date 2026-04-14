use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::rc::Rc;

use crate::simulation_times::SimulationTimes;

use ndarray::{Array1, Array2, Array3, Array4, Axis};
use ndarray_linalg::trace::Trace;
use num_complex::{ Complex64};

pub struct SimulationResults {
    simulation_times: Rc<SimulationTimes>,
    density_matrices: Array3<Complex64>,
}

impl SimulationResults {
    pub fn new(
        simulation_times: Rc<SimulationTimes>,
        starting_density_matrx: &Array2<Complex64>,
    ) -> SimulationResults {
        let num_samples = simulation_times.get_num_samples();
        let mut density_matrices = Array3::<Complex64>::zeros([num_samples, 2, 2]);
        density_matrices
            .index_axis_mut(Axis(0), 0)
            .assign(starting_density_matrx);
        return SimulationResults {
            simulation_times: simulation_times,
            density_matrices: density_matrices,
        };
    }
    pub fn evolve_state(
        &mut self,
        sample_num: usize,
        evolution_operators: Array4<Complex64>,
    ) -> () {
        // Copy the last state to evolve as the next sampled state
        let mut next_sample: Array2<Complex64> = self
            .density_matrices
            .index_axis(Axis(0), sample_num)
            .clone()
            .to_owned();
        for iter in evolution_operators.outer_iter() {
            next_sample = iter
                .index_axis(Axis(0), 0)
                .dot(&next_sample.dot(&iter.index_axis(Axis(0), 1)));
        }
        self.density_matrices
            .index_axis_mut(Axis(0), sample_num + 1)
            .assign(&next_sample);
        return;
    }
    pub fn get_density_matrices(&self) -> &Array3<Complex64> {
        return &self.density_matrices;
    }
    pub fn get_simulation_times(&self) -> Rc<SimulationTimes> {
        return Rc::clone(&self.simulation_times);
    }
    pub fn get_probability(&self, sample_num: usize, state: &Array1<Complex64>) -> f64 {
        let projection_operator: Array2<Complex64> = state
            .to_shape([2, 1])
            .unwrap()
            .dot(&state.mapv(|x| x.conj()).to_shape([1, 2]).unwrap());
        return self
            .density_matrices
            .index_axis(Axis(0), sample_num)
            .dot(&projection_operator)
            .trace()
            .unwrap()
            .re;
    }
    pub fn get_final_state_probability(&self, state: &Array1<Complex64>) -> f64 {
        return self.get_probability(self.density_matrices.shape()[0] - 1, state);
    }
    pub fn get_final_probability(&self) -> f64 {
        return self.get_final_state_probability(&Array1::<Complex64>::from_vec(vec![
            Complex64::new(0., 0.),
            Complex64::new(0., 1.),
        ]));
    }
    pub fn get_state_probabilities(&self, state: &Array1<Complex64>) -> Array1<f64> {
        let mut probabilities: Array1<f64> = Array1::<f64>::zeros([self.simulation_times.get_num_samples()]);
        for i in 0..self.simulation_times.get_num_samples() {
            probabilities[[i]] = self.get_probability(i, state);
        }
        return  probabilities;
    }
    pub fn get_probabilities(&self) -> Array1<f64> {
        return self.get_state_probabilities(&Array1::<Complex64>::from_vec(vec![Complex64::new(0., 0.), Complex64::new(1., 0.)]));
    }
    pub fn save_bloch_coords_cart(&self, file_name: &str) -> Result<()> {
        let write_file: File = File::create(file_name).unwrap();
        let mut writer: BufWriter<&File> = BufWriter::new(&write_file);

        for i in 0..self.density_matrices.shape()[0] {
            let coords: (f64, f64, f64) = self.get_bloch_coords_cart(i);
            write!(&mut writer, "{0}, {1}, {2}\n", coords.0, coords.1, coords.2)?;
        }

        return Ok(());
    }
    pub fn get_bloch_coords_cart(&self, sample_num: usize) -> (f64, f64, f64) {
        return (
            2. * self.density_matrices[[sample_num, 1, 0]].re,
            2. * self.density_matrices[[sample_num, 1, 0]].im,
            2. * self.density_matrices[[sample_num, 0, 0]].re - 1.,
        );
    }
}
