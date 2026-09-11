use crate::experiment::SweepParameter;
use crate::simulation::LarmorFrequency;

use serde_json::{Map, Value};

#[derive(Debug)]
pub struct LarmorFrequencyBlueprint {
    base_value: f64,
    pink_noise_power: f64,
}

impl LarmorFrequencyBlueprint {
    pub fn from_json(
        mut json_values: Map<String, Value>,
    ) -> (LarmorFrequencyBlueprint, Vec<SweepParameter>) {
        // Empty vector for the sweep parameters
        let mut swept_parameters: Vec<SweepParameter> = vec![];

        // Store the larmor and guess lamrmor
        let (base_value, sweep_parameter_option): (f64, Option<SweepParameter>) =
            SweepParameter::from_json(
                "base_value",
                serde_json::from_value(json_values.remove("base_value").unwrap()).unwrap(),
            );
        if let Some(sweep_parameter) = sweep_parameter_option {
            swept_parameters.push(sweep_parameter);
        }

        let (pink_noise_power, sweep_parameter_option): (f64, Option<SweepParameter>) =
            SweepParameter::from_json(
                "pink_noise_power",
                serde_json::from_value(json_values.remove("pink_noise_power").unwrap()).unwrap(),
            );
        if let Some(sweep_parameter) = sweep_parameter_option {
            swept_parameters.push(sweep_parameter);
        }

        return (
            LarmorFrequencyBlueprint {
                base_value: base_value,
                pink_noise_power: pink_noise_power,
            },
            swept_parameters,
        );
    }
    pub fn get_larmor_frequency(&self) -> LarmorFrequency {
        return LarmorFrequency::new(self.base_value, self.pink_noise_power, 0.);
    }
    /// Update the parameters for this blueprint
    pub fn update_parameters(
        &mut self,
        sweep_parameter: &SweepParameter,
        path_index: usize,
        value_index: usize,
    ) -> () {
        // Match the path to be updated and update it
        match sweep_parameter.get_path(path_index).as_str() {
            "base_value" => self.base_value = sweep_parameter.get_value(value_index),
            "pink_noise_power" => self.pink_noise_power = sweep_parameter.get_value(value_index),
            _ => return,
        }
        return;
    }
}
