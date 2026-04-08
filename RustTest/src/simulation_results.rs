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
}
