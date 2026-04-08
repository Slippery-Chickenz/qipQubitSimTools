use std::f64::consts::PI;
use std::rc::Rc;

use crate::gate::Gate;
use crate::simulation_times::SimulationTimes;

use num_complex::Complex64;
use ndarray::Array3;

pub struct Circuit {
    gates: Vec<Box<dyn Gate>>,
    duration: f64,
    integrated_frequencies: Vec<Vec<f64>>,
    simulation_times: Rc<SimulationTimes>,
}

impl Circuit {
    pub fn new() -> Circuit {
        return Circuit { gates: vec![], duration: 0., integrated_frequencies: vec![], simulation_times: Rc::new(SimulationTimes::new(10., 3, 2))}
    }
    pub fn get_duration(&self) -> f64 {
        return self.duration;
    }
    pub fn get_num_qubits(&self) -> u32 {
        return 1;
    }
    pub fn add_gate(&mut self, gate: Box<dyn Gate>) -> () {
        self.duration += gate.get_duration();
        self.gates.push(gate);
    }
    pub fn set_simulation_times(&mut self, times: Rc<SimulationTimes>) -> () {
        self.simulation_times = times;
        self.integrate_frequencies();
        return;
    }
    pub fn integrate_frequencies(&mut self) -> () {

        // Set the integrated frequencies to an array with the size of the times
        self.integrated_frequencies = Vec::with_capacity(self.simulation_times.get_num_sample_times());

        // Temporary frequency that is the integrated value
        let mut temp_f: f64 = 0.;

        // Integrate through all the frequencies
        for i in 0..self.simulation_times.get_num_sample_times()-1 {

            // Temporary vector to hold integrated frequencies for this sample
            let mut temp_fs: Vec<f64> = Vec::with_capacity(self.simulation_times.get_num_iterations_per_sample());
            for t in self.simulation_times.get_iteration_times_after_sample(i) {
                temp_f += self.get_frequency(*t);
                temp_fs.push(temp_f);
            }
            self.integrated_frequencies.push(temp_fs);
        }
        return;
    }
    pub fn get_hamiltonian_operator(&self, sample_num: usize) -> Array3<Complex64> {

        // Array of hamiltonians at each time step
        let mut hamiltonians: Array3<Complex64> = Array3::<Complex64>::zeros([self.integrated_frequencies[sample_num].len(), 2, 2]);


        for (i, t) in self.simulation_times.get_iteration_times_after_sample(sample_num).iter().enumerate() {

            let amplitude: f64 = self.get_amplitude(*t);
            let frequency: f64 = self.get_integrated_frequency(sample_num, i);
            let phase: f64 = self.get_phase(*t);

            hamiltonians[[i, 0, 1]] = Complex64::new( amplitude * PI * (2. * PI * frequency + phase).cos(),
                                                      amplitude * PI * (2. * PI * frequency + phase).sin() );
            hamiltonians[[i, 1, 0]] = Complex64::new( amplitude * PI * (2. * PI * frequency + phase).cos(),
                                                     -amplitude * PI * (2. * PI * frequency + phase).sin() );
        }

        return hamiltonians;
    }

    fn get_amplitude(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_amplitude(time)
    }
    fn get_frequency(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_frequency(time);
    }
    fn get_integrated_frequency(&self, sample_num: usize, time_index: usize) -> f64 {
        return self.integrated_frequencies[sample_num][time_index];
    }
    fn get_phase(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_phase(time);
    }
    fn get_gate_index(&self, mut time: f64) -> usize {
        
        for (i, gate) in self.gates.iter().enumerate() {
            let gate_duration: f64 = gate.get_duration();
            if time <= gate_duration {
                return i;
            }
            time -= gate_duration;
        }
        panic!("Tried to get gate index for time past the durration of the circuit.")
    }
}
