use crate::simulation::{ReferenceFrame, SimulationSettings};

use serde_json::{Map, Value};

#[derive(Debug)]
pub enum SimulationMethodType {
    RKMethod,
    RKVectorizedMethod,
    PadeVectorizedMethod,
    PadeStateMethod,
}

impl From<String> for SimulationMethodType {
    fn from(value: String) -> SimulationMethodType {
        match value.as_str() {
            "RK" => return SimulationMethodType::RKMethod,
            "RKVectorized" => return SimulationMethodType::RKVectorizedMethod,
            "PadeVectorized" => return SimulationMethodType::PadeVectorizedMethod,
            "PadeState" => return SimulationMethodType::PadeStateMethod,
            _ => panic!("Invalid simulation method!"),
        }
    }
}

/// Blueprint to construct a set of simulation times
#[derive(Debug)]
pub struct SimulationSettingsBlueprint {
    /// Step size for each iteration
    step_size: f64,
    /// Number of samples to save
    num_samples: usize,
    /// Number of shots to perform for this simulation
    num_shots: usize,
    /// Simulation method to run the experiment with
    method: SimulationMethodType,
    /// Reference frame for simulation
    frame: ReferenceFrame,
    /// Whether or not to save the hamiltonian at each sample
    save_hamiltonian: bool,
}

impl SimulationSettingsBlueprint {
    /// Get a SimulationTimesBlueprint object from a map of Strings to json values
    pub fn from_json(mut json_values: Map<String, Value>) -> SimulationSettingsBlueprint {
        // Just get the number of iterations and samples as u64 from the map
        return SimulationSettingsBlueprint {
            step_size: serde_json::from_value(json_values.remove("step_size").unwrap()).unwrap(),
            num_samples: serde_json::from_value(json_values.remove("num_samples").unwrap())
                .unwrap(),
            num_shots: serde_json::from_value(json_values.remove("num_shots").unwrap()).unwrap(),
            method: SimulationMethodType::from(
                serde_json::from_value::<String>(json_values.remove("method").unwrap()).unwrap(),
            ),
            frame: ReferenceFrame::from(
                serde_json::from_value::<String>(json_values.remove("frame").unwrap()).unwrap(),
            ),
            save_hamiltonian: false,
        };
    }
    pub fn save_hamiltonians(&mut self, save_hamiltonians: bool) -> () {
        self.save_hamiltonian = save_hamiltonians;
        return;
    }
    /// Get the simulation settings for this blueprint
    pub fn get_simulation_settings(&self) -> SimulationSettings {
        return SimulationSettings::new(
            self.step_size,
            self.num_samples,
            self.num_shots,
            self.frame.clone(),
            self.save_hamiltonian,
        );
    }
    /// Get the number of samples from the blueprint
    pub fn get_num_samples(&self) -> usize {
        return self.num_samples;
    }
    /// Get the number of shots from the blueprint
    pub fn get_num_shots(&self) -> usize {
        return self.num_shots;
    }
    /// Get the step size for the blueprint
    pub fn get_step_size(&self) -> f64 {
        return self.step_size;
    }
    /// Get the simulation method
    pub fn get_simulation_method(&self) -> &SimulationMethodType {
        return &self.method;
    }
    /// Get the reference frame
    pub fn get_reference_frame(&self) -> &ReferenceFrame {
        return &self.frame;
    }
}
