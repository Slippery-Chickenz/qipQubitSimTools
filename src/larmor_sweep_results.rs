use std::fs::File;
use std::io::{BufWriter, Result, Write};

use crate::simulation_results::SimulationResults;

use ndarray::{Array1, array};
use num_complex::Complex64;

pub struct LarmorSweepResult {
    simulation_results: Vec<SimulationResults>,
    larmor_values: Vec<f64>,
}

impl LarmorSweepResult {
    pub fn new() -> LarmorSweepResult {
        return LarmorSweepResult {
            simulation_results: vec![],
            larmor_values: vec![],
        };
    }
    pub fn set_larmor_values(&mut self, larmor_values: Vec<f64>) -> () {
        self.larmor_values = larmor_values;
    }
    pub fn add_result(&mut self, simulation_result: SimulationResults) -> () {
        self.simulation_results.push(simulation_result);
    }
    pub fn save_state_probability(&self, file_name: &str, state: Array1<Complex64>) -> Result<()> {
        let write_file: File = File::create(file_name).unwrap();
        let mut writer: BufWriter<&File> = BufWriter::new(&write_file);

        for (i, simulation_result) in self.simulation_results.iter().enumerate() {
            write!(
                &mut writer,
                "{0}, {1}\n",
                self.larmor_values[i],
                simulation_result.get_final_state_probability(&state)
            )?;
        }
        return Ok(());
    }
    pub fn save_probability(&self, file_name: &str) -> Result<()> {
        self.save_state_probability(
            file_name,
            array![Complex64::new(1., 0.), Complex64::new(0., 0.)],
        )?;
        return Ok(());
    }
}
