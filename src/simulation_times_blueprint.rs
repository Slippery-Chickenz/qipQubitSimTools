use serde_json::{Map, Value};

#[derive(Debug)]
pub struct SimulationTimesBlueprint {
    num_iterations: usize,
    num_samples: usize,
}

impl SimulationTimesBlueprint {
    pub fn from_json(json_values: &Map<String, Value>) -> SimulationTimesBlueprint {
        return SimulationTimesBlueprint { num_iterations: json_values["num_iterations"].as_u64().unwrap() as usize, num_samples: json_values["num_samples"].as_u64().unwrap() as usize };
    }
    pub fn get_num_iterations(&self) -> usize {
        return self.num_iterations;
    }
    pub fn get_num_samples(&self) -> usize {
        return self.num_samples;
    }
}
