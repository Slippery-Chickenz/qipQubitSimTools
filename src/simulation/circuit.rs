use std::f64::consts::PI;

use crate::gates::Gate;
use crate::simulation::SimulationTimes;

use ndarray::{Array1, Array2};

/// Quantum circuit to be simulated
pub struct Circuit {
    /// Vector of objects which implement the (Quantum) Gate trait
    gates: Vec<Box<dyn Gate>>,
    /// Duration of the circuit. Just the sum of the duration of each gate
    duration: f64,
    /// For the circuit to be simulated the frequency of each gate must be integrated over all the
    /// time steps.
    frequency: f64,
    time: f64,
}

impl Circuit {
    /// Create a new empty circuit object
    pub fn new() -> Circuit {
        return Circuit {
            gates: vec![],
            duration: 0.,
            frequency: 0.,
            time: 0.,
        };
    }
    /// Create a new circuit object with the gates given in a vector
    /// Equivalent to `circuit::new();` followed by `circuit.add_gates(gates);`
    pub fn from_vec(gates: Vec<Box<dyn Gate>>) -> Circuit {
        let mut circuit: Circuit = Circuit::new();
        circuit.add_gates(gates);
        return circuit;
    }
    /// Get the duration of the entire circuit
    pub fn get_duration(&self) -> f64 {
        return self.duration;
    }
    /// Get the number of qubits this gate is for (currently only supports 1)
    pub fn get_num_qubits(&self) -> u32 {
        return 1;
    }
    /// Add a gate to the end of the circuit
    pub fn add_gate(&mut self, gate: Box<dyn Gate>) -> () {
        self.duration += gate.get_duration();
        self.gates.push(gate);
    }
    /// Add a vector of gates onto the end of the circuit
    pub fn add_gates(&mut self, mut gates: Vec<Box<dyn Gate>>) -> () {
        for gate in &gates {
            self.duration += gate.get_duration();
        }
        self.gates.append(&mut gates);
        return;
    }
    /// Get the data needed to plot out the circuit. Returns 4 values: times for each data point,
    /// frequency data, amplitude_data, and combined pulse data (Real values of (0, 1) matrix
    /// element)
    pub fn get_circuit_data(&self) -> (Array1<f64>, Array1<f64>, Array1<f64>, Array2<f64>) {
        let times: SimulationTimes = SimulationTimes::new(self.duration, 0.001, 1, 2);

        // Empty vectors to store data
        let mut frequency_data: Array1<f64> =
            Array1::<f64>::zeros(times.get_iteration_times().shape()[0]);
        let mut amplitude_data: Array1<f64> =
            Array1::<f64>::zeros(times.get_iteration_times().shape()[0]);
        let mut phase_data: Array1<f64> =
            Array1::<f64>::zeros(times.get_iteration_times().shape()[0]);
        let mut pulse_data: Array2<f64> =
            Array2::<f64>::zeros((2, times.get_iteration_times().shape()[0]));

        // Integrated frequency for modulation
        let mut integrated_frequency: f64 = 0.;

        // Loop over each time step and compile frequency, amplitude, and pulse data
        for (i, t) in times.get_iteration_times().iter().enumerate() {
            frequency_data[i] = self.get_frequency(*t);
            integrated_frequency += frequency_data[i] * times.get_dt();
            amplitude_data[i] = self.get_amplitude(*t);
            phase_data[i] = self.get_phase(*t);
            pulse_data[[0, i]] = amplitude_data[i]
                * PI
                * 0.5
                * (2. * PI * integrated_frequency + phase_data[i]).cos();
            pulse_data[[1, i]] = amplitude_data[i]
                * PI
                * 0.5
                * (2. * PI * integrated_frequency + phase_data[i]).sin();
        }
        return (frequency_data, amplitude_data, phase_data, pulse_data);
    }
    /// Saves the circuit data to an HDF5 file to be plotted elsewhere. HDF5 file just has 4 data
    /// sets, time steps, frequency data, amplitude data, and the combined pulse data.
    pub fn save_circuit_data(&mut self) -> () {
        // Get circuit data to save
        let (time_data, frequency_data, amplitude_data, pulse_data) = self.get_circuit_data();

        // Create the file
        let file = hdf5::File::create("circuit_data.h5").unwrap();

        // Make the builder and save each of the data
        let builder = file.new_dataset_builder();
        let _ds = builder
            .clone()
            .with_data(&time_data)
            .create("time_data")
            .unwrap();
        let _ds = builder
            .clone()
            .with_data(&frequency_data)
            .create("frequency_data")
            .unwrap();
        let _ds = builder
            .clone()
            .with_data(&amplitude_data)
            .create("amplitude_data")
            .unwrap();
        let _ds = builder.with_data(&pulse_data).create("pulse_data").unwrap();
        return;
    }
    // Get the pulse amplitude of the circuit at a time
    pub fn get_amplitude(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_amplitude(time);
    }
    // Get the raw pulse frequency of the circuit at a time
    pub fn get_frequency(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_frequency(time);
    }
    // Get the integrated frequency of the circuit at a time
    pub fn get_integrated_frequency(&mut self, time: f64) -> f64 {
        let t: f64 = time;
        self.frequency += self.get_frequency(t) * (t - self.time);
        self.time = t;
        return self.frequency;
    }
    // Get the phase of the circuit at a time
    pub fn get_phase(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_phase(time);
    }
    /// Reset the circuit class to simulate a shot
    pub fn reset_for_shot(&mut self) -> () {
        self.time = 0.;
        self.frequency = 0.;
        return;
    }
    // Get the index in the gates vector of the gate which is playing at a given time
    fn get_gate_index(&self, time: f64) -> usize {
        let mut gate_time: f64 = time.clone();
        for (i, gate) in self.gates.iter().enumerate() {
            let gate_duration: f64 = gate.get_duration();
            if (gate_time - gate_duration) <= 1e-10 {
                return i;
            }
            gate_time -= gate_duration;
        }
        panic!(
            "Tried to get gate index for time past the durration of the circuit. Time: {}, Duration: {}",
            time, self.duration
        );
    }
}
