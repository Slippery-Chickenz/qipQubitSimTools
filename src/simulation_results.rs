use std::fs::File;
use std::io::{BufWriter, Write, Result};

use crate::qubit_array::QubitArray;

pub struct SimulationResults {
    qubit_array: QubitArray,
}

impl SimulationResults {
    pub fn new() -> SimulationResults {
        return SimulationResults { qubit_array: QubitArray::new(1, 0.) }
    }
    pub fn add_array(&mut self, qubit_array: &QubitArray) -> () {
        self.qubit_array = qubit_array.clone();
    }
    pub fn get_array(&self) -> &QubitArray {
        return &self.qubit_array;
    }
    pub fn save_bloch_coords_cart(&self, file_name: &str) -> Result<()> {

        let write_file: File = File::create(file_name).unwrap();
        let mut writer: BufWriter<&File> = BufWriter::new(&write_file);

        for i in 0..self.qubit_array.get_density_matrices().len() {
            let coords: (f64, f64, f64) = self.qubit_array.get_bloch_coords_cart(i);
            write!(&mut writer, "{0}, {1}, {2}\n", coords.0, coords.1, coords.2)?;
        }

        return Ok(());
    }
}
