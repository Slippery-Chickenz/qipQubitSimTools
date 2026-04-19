use crate::{qubit_array::QubitArray, sweep_parameter::SweepParameter};

use serde_json::{Map, Value};

/// Blueprint for constructing a qubit array to simulate.
#[derive(Debug)]
pub struct QubitArrayBlueprint {
    /// Larmor value for the qubit
    larmor: f64,
    /// Guess larmor for the qubit
    guess_larmor: f64,
}

impl QubitArrayBlueprint {
    /// Get a QubitArrayBlueprint object from a Map of Strings to json values. Returns not just the
    /// blueprint but also a vector of parameters to be swept over
    pub fn from_json(
        json_values: &Map<String, Value>,
    ) -> (QubitArrayBlueprint, Vec<SweepParameter>) {
        // Values for the first (and only as of now) qubit
        let q1_values: &Map<String, Value> = json_values["q1"].as_object().unwrap();

        // Empty vector for the sweep parameters
        let mut swept_parameters: Vec<SweepParameter> = vec![];

        // Store the larmor and guess lamrmor
        let larmor: f64;
        let guess_larmor: f64;

        // Get the larmor value from the map under the "larmor" key. If it is not a number then
        // assume it is an array and it must be swept over
        if !q1_values["larmor"].is_number() {
            swept_parameters.push(SweepParameter::from_json(
                "larmor".to_string(),
                &q1_values["larmor"],
            ));
            // If it is an array to sweep then just set it to the first item in the array to start
            larmor = swept_parameters[swept_parameters.len() - 1].get_value(0);
        } else {
            larmor = q1_values["larmor"].as_f64().unwrap();
        }

        // Same but for guess larmor. If it is not a number it must be an array to sweep over
        if !q1_values["guess_larmor"].is_number() {
            swept_parameters.push(SweepParameter::from_json(
                "guess_larmor".to_string(),
                &q1_values["guess_larmor"],
            ));
            guess_larmor = swept_parameters[swept_parameters.len() - 1].get_value(0);
        } else {
            guess_larmor = q1_values["guess_larmor"].as_f64().unwrap();
        }

        return (
            QubitArrayBlueprint {
                larmor: larmor,
                guess_larmor: guess_larmor,
            },
            swept_parameters,
        );
    }
    /// Update the parameters for this blueprint
    pub fn update_parameters(
        &mut self,
        sweep_parameter: &SweepParameter,
        path_index: usize,
        value_index: usize,
    ) -> () {
        // Match the path to be updated with either the guess larmor or the larmor and set it
        match sweep_parameter.get_path(path_index).as_str() {
            "guess_larmor" => self.guess_larmor = sweep_parameter.get_value(value_index),
            "larmor" => self.larmor = sweep_parameter.get_value(value_index),
            _ => return,
        }
        return;
    }
    /// Get a qubit array object constructed from this blueprint
    pub fn get_qubit_array(&self) -> QubitArray {
        return QubitArray::new(1, self.larmor, self.guess_larmor);
    }
}
