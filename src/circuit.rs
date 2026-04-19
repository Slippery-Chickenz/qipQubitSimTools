use std::f64::consts::PI;
use std::option::Option;
use std::rc::Rc;

use crate::gate::Gate;
use crate::simulation_times::{SimulationTimes, UninitializedTimesError};

use ndarray::{Array1, Array2, Array3};
use num_complex::Complex64;

/// Quantum circuit to be simulated
pub struct Circuit {
    /// Vector of objects which implement the (Quantum) Gate trait
    gates: Vec<Box<dyn Gate>>,
    /// Duration of the circuit. Just the sum of the duration of each gate
    duration: f64,
    /// Simulation times to run the circuit at.
    simulation_times: Option<Rc<SimulationTimes>>,
    /// For the circuit to be simulated the frequency of each gate must be integrated over all the
    /// time steps. This is a vector of vectors. The outer vector is the sample number those
    /// frequencies are for. The inner vector is for the specific time step.
    integrated_frequencies: Array2<f64>,
}

impl Circuit {
    /// Create a new empty circuit object
    pub fn new() -> Circuit {
        return Circuit {
            gates: vec![],
            duration: 0.,
            integrated_frequencies: Array2::<f64>::zeros([1, 1]),
            simulation_times: None,
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
    /// Set the simulation times for the circuit and calculate the integrated frequencies for these
    /// simulation times.
    pub fn set_simulation_times(&mut self, times: Rc<SimulationTimes>) -> () {
        self.simulation_times = Some(times);
        self.integrate_frequencies();
        return;
    }
    /// Integrate the frequencies of the circuit based on simulation times. This integration needs
    /// to happen for any frequency modulation. The simulation is done in a rotating frame rotating
    /// at a frequency such that the carrier frequency of any gate is 0.
    ///
    /// Integrated frequency values are stored in a 2D Vector. This is due to how the simulation
    /// times are set as a number of samples and an number of iterations per sample. The first axis
    /// of the integrated frequencies is the sample number and the second axis is the iteration
    /// number for that sample.
    fn integrate_frequencies(&mut self) -> () {
        // Cannot integrate frequencies without simulation times
        if let Some(sim_times) = &self.simulation_times {
            self.integrated_frequencies = Array2::<f64>::zeros([
                sim_times.get_num_samples() - 1,
                sim_times.get_num_iterations_per_sample(),
            ]);
            let mut temp_f: f64 = 0.; // Temporary frequency that is the integrated value

            // Outer axis is looped over number of samples
            for i in 0..sim_times.get_num_samples() - 1 {
                // Inner axis loops over the number of iterations for this sample
                for (j, t) in sim_times
                    .get_iteration_times_after_sample(i)
                    .iter()
                    .enumerate()
                {
                    temp_f += self.get_frequency(*t);
                    self.integrated_frequencies[[i, j]] = temp_f;
                }
            }
            // Multiply by dt for integration
            self.integrated_frequencies *= sim_times.get_dt();
        } else {
            eprintln!("{}", UninitializedTimesError);
        }
        return;
    }
    /// Get hamiltonian operator for every iteration for a specific sample. If the sample number
    /// given is n then this will return the hamiltonian operators for iterations between n and n+1.
    /// This hamiltonian is just the pulse component and is denoted as:
    /// $$ H = \frac{\Omega(t)}{2} 2\pi (\cos(2\pi \int f(t) + \phi(t))S_x + \sin(2\pi \int f(t) + \phi(t))S_y) $$
    pub fn get_hamiltonian_operator(&self, sample_num: usize) -> Array3<Complex64> {
        // Array of hamiltonians at each time step
        // Outer axis is the iteration number
        // Below that are the 2x2 Hamiltonians for the single qubit gates
        let mut hamiltonians: Array3<Complex64> =
            Array3::<Complex64>::zeros([self.integrated_frequencies.shape()[1], 2, 2]);

        // If simulation times are not set then we error
        if let Some(sim_times) = &self.simulation_times {
            // Loop through the index and time for the simulation times at this sample
            for (i, t) in sim_times
                .get_iteration_times_after_sample(sample_num)
                .iter()
                .enumerate()
            {
                // Amplitude, frequency, and phase for this time step in the circuit
                let amplitude: f64 = self.get_amplitude(*t);
                let frequency: f64 = self.get_integrated_frequency(sample_num, i);
                let phase: f64 = self.get_phase(*t);

                // Set hamiltonian values
                hamiltonians[[i, 0, 1]] = Complex64::new(
                    amplitude * PI * 0.5 * (2. * PI * frequency + phase).cos(),
                    amplitude * PI * 0.5 * (2. * PI * frequency + phase).sin(),
                );
                hamiltonians[[i, 1, 0]] = Complex64::new(
                    amplitude * PI * 0.5 * (2. * PI * frequency + phase).cos(),
                    -amplitude * PI * 0.5 * (2. * PI * frequency + phase).sin(),
                );
            }
        } else {
            eprintln!("{}", UninitializedTimesError);
        }

        return hamiltonians;
    }
    /// Get the data needed to plot out the circuit. Returns 4 values: times for each data point,
    /// frequency data, amplitude_data, and combined pulse data (Real values of (0, 1) matrix
    /// element)
    pub fn get_circuit_data(&mut self) -> (Array1<f64>, Array1<f64>, Array1<f64>, Array1<f64>) {
        // Set time steps to run at (10000 points is arbitrary)
        let time_steps: Array1<f64> = Array1::<f64>::linspace(0., self.duration, 10000);

        // Temporarily set the simulation times to get properly integrated frequencies
        self.set_simulation_times(Rc::new(SimulationTimes::new(
            self.duration,
            time_steps.len(),
            2,
        )));
        self.integrate_frequencies();

        // Empty vectors to store data
        let mut frequency_data: Array1<f64> = Array1::<f64>::zeros(time_steps.len());
        let mut amplitude_data: Array1<f64> = Array1::<f64>::zeros(time_steps.len());
        let mut pulse_data: Array1<f64> = Array1::<f64>::zeros(time_steps.len());

        // Loop over each time step and compile frequency, amplitude, and pulse data
        for (i, t) in time_steps.iter().enumerate() {
            frequency_data[i] = self.get_frequency(*t);
            amplitude_data[i] = self.get_amplitude(*t);
            pulse_data[i] = amplitude_data[i]
                * PI
                * 0.5
                * (2. * PI * self.get_integrated_frequency(0, i) + self.get_phase(*t)).cos();
        }

        // Reset the simulation times and integrated frequencies
        self.simulation_times = None;
        self.integrated_frequencies = Array2::<f64>::zeros([1, 1]);
        return (time_steps, frequency_data, amplitude_data, pulse_data);
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
    fn get_amplitude(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_amplitude(time);
    }
    // Get the raw pulse frequency of the circuit at a time
    fn get_frequency(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_frequency(time);
    }
    // Get the integrated frequency of the circuit at a time
    fn get_integrated_frequency(&self, sample_num: usize, time_index: usize) -> f64 {
        return self.integrated_frequencies[[sample_num, time_index]];
    }
    // Get the phase of the circuit at a time
    fn get_phase(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_phase(time);
    }
    // Get the index in the gates vector of the gate which is playing at a given time
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
