use std::f64::consts::PI;
use std::rc::Rc;

use crate::simulation::{LarmorFrequency, SimulationTimes, SimulationSettings};

use ndarray::{Array1, Array2};
use num_complex::Complex64;

/// Qubit array to be used in a simulation. Holds the number of qubits (currently only supports 1)
/// the starting density matrix of the qubits, the larmor of each qubit and the guess at what the
/// larmor is for each qubit
pub struct QubitArray {
    /// Number of qubits in the array (currently only supports 1)
    num_qubts: u32,
    /// Starting density matrix of the qubits
    starting_state: Array1<Complex64>,
    /// Larmor value of the qubit
    larmor: LarmorFrequency,
    /// Guess at the larmor value for the qubits
    guess_larmor: f64,
    /// Coefficient to determine how strong decoherence is
    decoherence: f64,
    // /// Simulation times for the simulation
    // simulation_times: Option<Rc<SimulationTimes>>,
}

impl QubitArray {
    /// Get a QubitArray object with a given number of qubits with a certain larmor and guess
    /// larmor. This sets the starting density matrix to be in the +z state e.g. (1, 0)
    pub fn new(
        num_qubits: u32,
        larmor: LarmorFrequency,
        guess_larmor: f64,
        decoherence: f64,
        starting_state: Array1<Complex64>,
    ) -> QubitArray {
        return QubitArray {
            num_qubts: num_qubits,
            starting_state: starting_state,
            larmor: larmor,
            guess_larmor: guess_larmor,
            decoherence: decoherence,
            // simulation_times: None,
        };
    }
    /// Set the simulation times for the qubit array
    pub fn initialize_larmor_noise(&mut self, simulation_settings: &SimulationSettings, simulation_times: &SimulationTimes) -> () {
        // self.simulation_times = Some(Rc::clone(&simulation_times));
        self.larmor.calculate_noise_values(simulation_settings, simulation_times);
        return;
    }
    /// Get the density_matrix that represents the starting state for the qubits
    pub fn get_starting_state(&self) -> &Array1<Complex64> {
        return &self.starting_state;
    }
    /// Get the number of qubits (currently only 1)
    pub fn get_num_qubits(&self) -> u32 {
        return self.num_qubts;
    }
    ///  Get the decoherence strength of the qubits
    pub fn get_decoherence(&self) -> f64 {
        return self.decoherence;
    }
    /// Get the larmor frequency of the qubits
    pub fn get_larmor_frequency(&self, shot_num: usize, iteration_index: usize) -> f64 {
        return self.larmor.get_larmor_frequency(shot_num, iteration_index);
    }
    /// Get the guess larmor frequency of the qubits
    pub fn get_guess_larmor(&self) -> f64 {
        return self.guess_larmor;
    }
}
