use crate::qubit_array::QubitArray;

pub struct SimulationResults {
    qubit_arrays: Vec<QubitArray>,
}

impl SimulationResults {
    pub fn new(num_iterations: usize) -> SimulationResults {
        return SimulationResults { qubit_arrays: Vec::with_capacity(num_iterations) }
    }
    pub fn add_array(&mut self, qubit_array: &QubitArray) -> () {
        self.qubit_arrays.push(qubit_array.clone());
    }
}
