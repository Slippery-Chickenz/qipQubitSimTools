use std::f64::consts::PI;
use std::option::Option;
use std::rc::Rc;

use crate::gate::Gate;
use crate::simulation_times::{SimulationTimes, UninitializedTimesError};

use ndarray::{Array1, Array3};
use num_complex::Complex64;

pub struct Circuit {
    gates: Vec<Box<dyn Gate>>,
    duration: f64,
    integrated_frequencies: Vec<Vec<f64>>,
    simulation_times: Option<Rc<SimulationTimes>>,
}

impl Circuit {
    pub fn new() -> Circuit {
        return Circuit {
            gates: vec![],
            duration: 0.,
            integrated_frequencies: vec![],
            simulation_times: None,
        };
    }
    pub fn from_vec(gates: Vec<Box<dyn Gate>>) -> Circuit {
        let mut circuit: Circuit = Circuit::new();
        circuit.add_gates(gates);
        return circuit;
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
    pub fn add_gates(&mut self, mut gates: Vec<Box<dyn Gate>>) -> () {
        for gate in &gates {
            self.duration += gate.get_duration();
        }
        self.gates.append(&mut gates);
        return;
    }
    pub fn set_simulation_times(&mut self, times: Rc<SimulationTimes>) -> () {
        self.simulation_times = Some(times);
        self.integrate_frequencies();
        return;
    }
    fn integrate_frequencies(&mut self) -> () {
        if let Some(sim_times) = &self.simulation_times {
            // Set the integrated frequencies to an array with the size of the times
            self.integrated_frequencies = Vec::with_capacity(sim_times.get_num_samples());

            // Temporary frequency that is the integrated value
            let mut temp_f: f64 = 0.;

            // Integrate through all the frequencies
            for i in 0..sim_times.get_num_samples() - 1 {
                // Temporary vector to hold integrated frequencies for this sample
                let mut temp_fs: Vec<f64> =
                    Vec::with_capacity(sim_times.get_num_iterations_per_sample());
                for t in sim_times.get_iteration_times_after_sample(i) {
                    temp_f += self.get_frequency(*t);
                    temp_fs.push(temp_f * sim_times.get_dt());
                }
                self.integrated_frequencies.push(temp_fs);
            }
        } else {
            eprintln!("{}", UninitializedTimesError);
        }
        return;
    }
    pub fn get_hamiltonian_operator(&self, sample_num: usize) -> Array3<Complex64> {
        // Array of hamiltonians at each time step
        let mut hamiltonians: Array3<Complex64> =
            Array3::<Complex64>::zeros([self.integrated_frequencies[sample_num].len(), 2, 2]);

        for (i, t) in self
            .simulation_times
            .as_ref()
            .ok_or(UninitializedTimesError)
            .unwrap()
            .get_iteration_times_after_sample(sample_num)
            .iter()
            .enumerate()
        {
            let amplitude: f64 = self.get_amplitude(*t);
            let frequency: f64 = self.get_integrated_frequency(sample_num, i);
            let phase: f64 = self.get_phase(*t);

            hamiltonians[[i, 0, 1]] = Complex64::new(
                amplitude * PI * 0.5 * (2. * PI * frequency + phase).cos(),
                amplitude * PI * 0.5 * (2. * PI * frequency + phase).sin(),
            );
            hamiltonians[[i, 1, 0]] = Complex64::new(
                amplitude * PI * 0.5 * (2. * PI * frequency + phase).cos(),
                -amplitude * PI * 0.5 * (2. * PI * frequency + phase).sin(),
            );
        }

        return hamiltonians;
    }
    pub fn get_circuit_data(&mut self) -> (Array1<f64>,Array1<f64>,Array1<f64>,Array1<f64>) {

        let time_steps: Array1<f64> = Array1::<f64>::linspace(0., self.duration, 10000);

        self.set_simulation_times(Rc::new(SimulationTimes::new(self.duration, time_steps.len(), 2)));
        self.integrate_frequencies();

        let mut frequency_data: Array1<f64> = Array1::<f64>::zeros(time_steps.len());
        let mut amplitude_data: Array1<f64> = Array1::<f64>::zeros(time_steps.len());
        let mut pulse_data: Array1<f64> = Array1::<f64>::zeros(time_steps.len());

        for (i, t) in time_steps.iter().enumerate() {
            frequency_data[i] = self.get_frequency(*t);
            amplitude_data[i] = self.get_amplitude(*t);
            pulse_data[i] = amplitude_data[i] * PI * 0.5 * (2. * PI * self.get_integrated_frequency(0, i)+self.get_phase(*t)).cos();
        }

        self.simulation_times = None;
        self.integrated_frequencies = vec![];
        return (time_steps, frequency_data, amplitude_data, pulse_data);
    }
    pub fn save_circuit_data(&mut self) -> () {

        let (time_data, frequency_data, amplitude_data, pulse_data) = self.get_circuit_data();

        let file = hdf5::File::create("circuit_data.h5").unwrap(); // open for writing

        let builder = file.new_dataset_builder();
        let _ds = builder
            .with_data(&time_data)
            // finalize and write the dataset
            .create("time_data").unwrap();

        let builder = file.new_dataset_builder();
        let _ds = builder
            .with_data(&frequency_data)
            // finalize and write the dataset
            .create("frequency_data").unwrap();

        let builder = file.new_dataset_builder();
        let _ds = builder
            .with_data(&amplitude_data)
            // finalize and write the dataset
            .create("amplitude_data").unwrap();

        let builder = file.new_dataset_builder();
        let _ds = builder
            .with_data(&pulse_data)
            // finalize and write the dataset
            .create("pulse_data").unwrap();

        return;
    }
    fn get_amplitude(&self, time: f64) -> f64 {
        return self.gates[self.get_gate_index(time)].get_amplitude(time);
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
