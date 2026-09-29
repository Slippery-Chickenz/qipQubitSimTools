use crate::simulation::ReferenceFrame;

/// Struct to hold the times for a simulation and the indices of the time values at which the
/// samples should be saved.
#[derive(Debug)]
pub struct SimulationSettings {
    /// Step size for each iteration
    step_size: f64,
    /// Number of samples to save
    num_samples: usize,
    /// Number of shots to perform for this simulation
    num_shots: usize,
    /// Reference frame for simulation
    frame: ReferenceFrame,
    /// Whether to save the hamiltonian or not
    save_hamiltonian: bool,
    /// Whether to save the larmor values used in the simulation
    save_larmors: bool,
}

impl SimulationSettings {
    /// Make a new SimulationTimes object given a duration, step size and number of samples to save
    pub fn new(
        step_size: f64,
        num_samples: usize,
        num_shots: usize,
        reference_frame: ReferenceFrame,
        save_hamiltonian: bool,
        save_lamors: bool,
    ) -> SimulationSettings {
        return SimulationSettings {
            step_size: step_size,
            num_samples: num_samples,
            num_shots: num_shots,
            frame: reference_frame,
            save_hamiltonian: save_hamiltonian,
            save_larmors: save_lamors,
        };
    }
    /// Get the dt for each time step
    pub fn get_dt(&self) -> f64 {
        return self.step_size;
    }
    /// Get the number of samples that are saved
    pub fn get_num_samples(&self) -> usize {
        return self.num_samples;
    }
    /// Get the number of shots that are taken
    pub fn get_num_shots(&self) -> usize {
        return self.num_shots;
    }
    /// Get the reference frame for the simulation
    pub fn get_reference_frame(&self) -> &ReferenceFrame {
        return &self.frame;
    }
    /// Get if the hamiltonian should be saved at each sample
    pub fn get_save_hamiltonian(&self) -> bool {
        return self.save_hamiltonian;
    }
    /// Get if the larmor values should be saved for the simulation
    pub fn get_save_lamors(&self) -> bool {
        return self.save_larmors;
    }
}
