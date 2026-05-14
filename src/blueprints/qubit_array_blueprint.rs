// use crate::blueprints::LarmorFrequencyBlueprint;
use crate::experiment::SweepParameter;
use crate::simulation::QubitArray;

use serde_json::{Map, Value};

/// Blueprint for constructing a qubit array to simulate.
#[derive(Debug)]
pub struct QubitArrayBlueprint {
    /// Larmor value for the qubit
    // larmor: LarmorFrequencyBlueprint,
    larmor: f64,
    /// Lifetime for the non-unitary continuous dephasing effect
    // dephasing_lifetime: f64,
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

        // Qubit Parameters
        let mut larmor: f64 = 0.0;
        let mut guess_larmor: f64 = 0.0;


        // Parameters to be swept over
        let mut swept_parameters: Vec<SweepParameter> = vec![];

        // Loop through all the keys and valeus in the json map given
        for (key, value) in q1_values.into_iter() {
            // Value to set the parameter to initially
            let inital_value: f64;
            
            // If value is not a number then it must be something to be swept over
            if !value.is_number() {
                // Add a new sweep parameter defined with the key name and the values in the json
                swept_parameters.push(SweepParameter::from_json(key.clone(), value));
                // Add the parameter to the blueprint with the value set to the first defined in
                // the sweep
                inital_value = swept_parameters[swept_parameters.len() - 1].get_value(0);
            } else {
                // If it is a number then just insert it into the blueprint
                inital_value = value.as_f64().unwrap();
            }
            match key.as_str() {
                "larmor" => larmor = inital_value,
                "guess_larmor" => guess_larmor = inital_value,
                _ => panic!("Bad Key"),
            }
        }

        //
        //
        //
        //
        //
        // // Empty vector for the sweep parameters
        // let mut swept_parameters: Vec<SweepParameter> = vec![];
        //
        // let (larmor, mut larmor_swept_parameters): (LarmorFrequencyBlueprint, Vec<SweepParameter>) =
        //     LarmorFrequencyBlueprint::from_json(q1_values["larmor"].as_object().unwrap());
        //
        //
        //
        // // Add to the path in the sweep parameter to track it for updates later
        // for sweep_parameter in &mut larmor_swept_parameters {
        //     sweep_parameter.add_path("larmor".to_string());
        // }
        // // Append the sweep parameters from this gate to the overall
        // swept_parameters.append(&mut larmor_swept_parameters);
        //
        // // Store the guess lamrmor
        // let guess_larmor: f64;
        //
        // // Same but for guess larmor. If it is not a number it must be an array to sweep over
        // if !q1_values["guess_larmor"].is_number() {
        //     swept_parameters.push(SweepParameter::from_json(
        //         "guess_larmor".to_string(),
        //         &q1_values["guess_larmor"],
        //     ));
        //     guess_larmor = swept_parameters[swept_parameters.len() - 1].get_value(0);
        // } else {
        //     guess_larmor = q1_values["guess_larmor"].as_f64().unwrap();
        // }
        //
        // // Store the dephasing_lifetime
        // let dephasing_lifetime: f64;
        //
        // // Same but for guess larmor. If it is not a number it must be an array to sweep over
        // if !q1_values["channels"]["dephasing_lifetime"].is_number() {
        //     swept_parameters.push(SweepParameter::from_json(
        //         "dephasing_lifetime".to_string(),
        //         &q1_values["channels"]["dephasing_lifetime"],
        //     ));
        //     dephasing_lifetime = swept_parameters[swept_parameters.len() - 1].get_value(0);
        // } else {
        //     dephasing_lifetime = q1_values["channels"]["dephasing_lifetime"]
        //         .as_f64()
        //         .unwrap();
        // }

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
        return QubitArray::new(
            1,
            self.larmor,
            self.guess_larmor,
        );
    }
}
