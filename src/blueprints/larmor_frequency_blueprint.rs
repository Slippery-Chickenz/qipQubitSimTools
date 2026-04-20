use crate::simulation::LarmorFrequency;
use crate::sweep_parameter::SweepParameter;

use serde_json::{Map, Value};

#[derive(Debug)]
pub struct LarmorFrequencyBlueprint {
    base_value: f64,
    pink_noise_power: f64,
}

impl LarmorFrequencyBlueprint {
    pub fn from_json(
        json_values: &Map<String, Value>,
    ) -> (LarmorFrequencyBlueprint, Vec<SweepParameter>) {
        // Values for the base value and noise
        // let larmor_values: &Map<String, Value> = json_values["q1"].as_object().unwrap();

        // Empty vector for the sweep parameters
        let mut swept_parameters: Vec<SweepParameter> = vec![];

        // Store the larmor and guess lamrmor
        let base_value: f64;
        let pink_noise_power: f64;

        let mut parameter_key: &str = "base_value";

        // Get the base value from the map under the "base_value" key. If it is not a number then
        // assume it is an array and it must be swept over
        if !json_values[parameter_key].is_number() {
            swept_parameters.push(SweepParameter::from_json(
                parameter_key.to_string(),
                &json_values[parameter_key],
            ));
            // If it is an array to sweep then just set it to the first item in the array to start
            base_value = swept_parameters[swept_parameters.len() - 1].get_value(0);
        } else {
            base_value = json_values[parameter_key].as_f64().unwrap();
        }

        parameter_key = "1/f_power";

        // Same but for guess larmor. If it is not a number it must be an array to sweep over
        if !json_values[parameter_key].is_number() {
            swept_parameters.push(SweepParameter::from_json(
                parameter_key.to_string(),
                &json_values[parameter_key],
            ));
            pink_noise_power = swept_parameters[swept_parameters.len() - 1].get_value(0);
        } else {
            pink_noise_power = json_values[parameter_key].as_f64().unwrap();
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
            "1/f_noise_power" => self.pink_noise_power = sweep_parameter.get_value(value_index),
            _ => return,
        }
        return;
    }
}
