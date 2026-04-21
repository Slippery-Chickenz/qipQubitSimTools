use serde_json::{Map, Value};

/// Blueprint to construct a set of simulation times
#[derive(Debug)]
pub struct SimulationTimesBlueprint {
    /// Number of iterations per sample
    num_iterations: usize,
    /// Number of samples to save
    num_samples: usize,
}

impl SimulationTimesBlueprint {
    /// Get a SimulationTimesBlueprint object from a map of Strings to json values
    pub fn from_json(json_values: &Map<String, Value>) -> SimulationTimesBlueprint {
        // Just get the number of iterations and samples as u64 from the map
        return SimulationTimesBlueprint {
            num_iterations: json_values["num_iterations"].as_u64().unwrap() as usize,
            num_samples: json_values["output"]["num_samples"].as_u64().unwrap() as usize,
        };
    }
    /// Get the number of iterations from the blueprint
    pub fn get_num_iterations(&self) -> usize {
        return self.num_iterations;
    }
    /// Get the number of samples from the blueprint
    pub fn get_num_samples(&self) -> usize {
        return self.num_samples;
    }
}
