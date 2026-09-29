use std::mem;

use crate::simulation::{SimulationSettings, SimulationTimes};

use ndarray::{Array1, Array2};
use num_complex::Complex64;
use rand::{RngExt, rng};
use rustfft::FftPlanner;

/// Hold the larmor frequency for a single two level system and any noise or movement that it may
/// contain.
#[derive(Debug)]
pub struct LarmorFrequency {
    /// Base value that is constant for the entire simulation
    base_larmor: f64,
    /// Power of 1/f noise added to the base value
    pink_noise_power: f64,
    /// Power of white noise added to the base value
    white_noise_power: f64,
    /// Larmor values at each iteration in the simulation
    // larmor_values: Array1<f64>,
    larmor_values: Array2<f64>,
    // Simulation times used to calculate the noise values
    // simulation_times: Option<Rc<SimulationTimes>>,
    // Simulation settings used to calculate the noise values
    // simulation_settings: SimulationSettings,
}

impl LarmorFrequency {
    pub fn new(base_larmor: f64, pink_noise_power: f64, white_noise_power: f64) -> LarmorFrequency {
        return LarmorFrequency {
            base_larmor: base_larmor,
            pink_noise_power: pink_noise_power,
            white_noise_power: white_noise_power,
            larmor_values: Array2::<f64>::zeros([0, 0]),
            // base_larmor,
            // pink_noise_power,
            // white_noise_power,
            // larmor_values: Array2::<f64>::zeros([0, 0]),
            // simulation_times: None,
            // simulation_settings: simulation_settings,
        };
    }
    pub fn get_larmor_frequency(&self, shot_num: usize, iteration_index: usize) -> f64 {
        return self.larmor_values[[shot_num, iteration_index]];
    }
    pub fn get_larmor_values(&self) -> &Array2<f64> {
        return &self.larmor_values;
    }
    pub fn take_larmor_values(&mut self) -> Array2<f64> {
        return mem::take(&mut self.larmor_values);
    }
    // pub fn get_larmor_frequency(&self, _time: f64) -> f64 {
    //     return self.base_larmor;
    // }
    // pub fn get_larmor_frequencies(&self) -> &Array1<f64> {
    //     return &self.larmor_values;
    // }
    /// Saves the larmor data to an HDF5 file to be plotted elsewhere. The HDF5 file just has 2 data
    /// sets, time steps and the larmor values
    // pub fn save_larmor_frequencies(&mut self, duration: f64, step_size: f64) -> () {
    //     // Temporarily set the simulation times to generate fake data
    //     self.set_simulation_times(Rc::new(SimulationTimes::new(duration, step_size, 1, 2)));
    //
    //     if let Some(sim_times) = &self.simulation_times {
    //         // Create the file
    //         let file = hdf5::File::create("larmor_data.h5").unwrap();
    //
    //         // Make the builder and save each of the data
    //         let builder = file.new_dataset_builder();
    //         let _ds = builder
    //             .clone()
    //             .with_data(sim_times.get_iteration_times())
    //             .create("time_data")
    //             .unwrap();
    //         let _ds = builder
    //             .clone()
    //             .with_data(&self.larmor_values)
    //             .create("larmor_data")
    //             .unwrap();
    //     }
    //     self.simulation_times = None;
    //     return;
    // }
    /// Set the simulation times and calculate the larmor noise
    // pub fn set_simulation_times(&mut self, simulation_times: Rc<SimulationTimes>) -> () {
    //     for _ in 0..simulation_times.get_num_samples() {
    //         self.larmor_values =
    //             Array1::<f64>::from_elem([simulation_times.get_num_iterations()], self.base_larmor);
    //     }
    //     self.simulation_times = Some(simulation_times);
    //     self.calculate_noise_values();
    //     return;
    // }
    // fn calculate_noise_values(&mut self) -> () {
    pub fn calculate_noise_values(
        &mut self,
        simulation_settings: &SimulationSettings,
        simulation_times: &SimulationTimes,
    ) -> () {
        let mut larmor_values: Array1<f64> = Array1::<f64>::from_elem(
            simulation_times.get_num_times() * simulation_settings.get_num_shots(),
            self.base_larmor,
        );
        if self.white_noise_power != 0. {
            larmor_values = self.calculate_white_noise_values(larmor_values);
        }
        if self.pink_noise_power != 0. {
            larmor_values =
                self.calculate_pink_noise_values(larmor_values, simulation_settings.get_dt());
        }
        self.larmor_values = larmor_values
            .into_shape_with_order([
                simulation_settings.get_num_shots(),
                simulation_times.get_num_times(),
            ])
            .unwrap();
        return;
    }
    fn calculate_white_noise_values(&self, mut larmor_values: Array1<f64>) -> Array1<f64> {
        let mut random_generator = rng();
        for i in 0..larmor_values.len() {
            let mut noise_offset: f64 = random_generator.random();
            noise_offset *= self.white_noise_power;
            larmor_values[i] += noise_offset;
        }
        return larmor_values;
    }
    fn calculate_pink_noise_values(
        &self,
        mut larmor_values: Array1<f64>,
        dt: f64,
    ) -> Array1<f64> {
        let mut random_generator = rng();
        let mut planner: FftPlanner<f64> = FftPlanner::new();
        let mut pink_noise_values: Vec<Complex64> =
            vec![Complex64::new(0., 0.); larmor_values.len()];
        let mut fft_frequency_scalings: Vec<f64> = vec![0.; pink_noise_values.len()];
        for i in 0..pink_noise_values.len() {
            pink_noise_values[i].re = random_generator.random();
            if i != 0 {
                fft_frequency_scalings[i] = (fft_frequency_scalings.len() as f64 * dt) / (i as f64);
            }
        }
        let fft = planner.plan_fft_forward(pink_noise_values.len());
        fft.process(&mut pink_noise_values);
        for i in 0..pink_noise_values.len() {
            pink_noise_values[i] *= fft_frequency_scalings[i];
        }
        let ifft = planner.plan_fft_inverse(pink_noise_values.len());
        ifft.process(&mut pink_noise_values);
        let maximum_pink_noise_value: f64 = pink_noise_values
            .iter()
            .max_by(|a, b| a.re.abs().total_cmp(&b.re.abs()))
            .unwrap()
            .re
            .abs();
        for i in 0..larmor_values.len() {
            larmor_values[i] +=
                (pink_noise_values[i].re * self.pink_noise_power) / maximum_pink_noise_value;
        }
        return larmor_values;
    }
}
