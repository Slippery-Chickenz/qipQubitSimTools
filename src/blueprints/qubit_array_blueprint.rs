use std::{fs, io::BufReader};

// use crate::blueprints::LarmorFrequencyBlueprint;
use crate::experiment::SweepParameter;
use crate::simulation::QubitArray;

use serde_json::{Map, Value};

/// Blueprint for constructing a qubit array to simulate.
#[derive(Debug)]
pub struct QubitArrayBlueprint {
    /// Larmor value for the qubit
    larmor: f64,
    /// Guess larmor for the qubit
    guess_larmor: f64,
    /// Decoherence strength for the qubit
    decoherence: f64,
}

impl QubitArrayBlueprint {
    /// Get a QubitArrayBlueprint object from a Map of Strings to json values. Returns not just the
    /// blueprint but also a vector of parameters to be swept over
    pub fn from_json(
        mut json_values: Map<String, Value>,
    ) -> (QubitArrayBlueprint, Vec<SweepParameter>) {

        // If there is a key in the map called "filename" then assume that is a seperate
        // file defining the circuit and look there
        if json_values.contains_key("filename") {
            let circuit_file: fs::File =
                fs::File::open(json_values["filename"].as_str().unwrap()).unwrap();
            let circuit_reader: BufReader<fs::File> = BufReader::new(circuit_file);
            json_values = serde_json::from_reader(circuit_reader).unwrap();
        }

        // Values for the first (and only as of now) qubit
        let q1_values: &Map<String, Value> = json_values["q1"].as_object().unwrap();

        // Qubit Parameters
        let mut larmor: f64 = 0.0;
        let mut guess_larmor: f64 = 0.0;
        let mut decoherence: f64 = 0.0;

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
                "decoherence" => decoherence = inital_value,
                _ => panic!("Bad Key"),
            }
        }
        return (
            QubitArrayBlueprint {
                larmor: larmor,
                guess_larmor: guess_larmor,
                decoherence: decoherence,
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
        return QubitArray::new(1, self.larmor, self.guess_larmor, self.decoherence);
    }
}
