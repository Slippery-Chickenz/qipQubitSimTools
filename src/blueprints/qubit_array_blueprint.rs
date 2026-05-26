// use crate::blueprints::LarmorFrequencyBlueprint;
use crate::simulation::QubitArray;
use crate::{blueprints::LarmorFrequencyBlueprint, experiment::SweepParameter};

use serde_json::{Map, Value};

/// Blueprint for constructing a qubit array to simulate.
#[derive(Debug)]
pub struct QubitArrayBlueprint {
    /// Larmor value for the qubit
    larmor: LarmorFrequencyBlueprint,
    /// Guess larmor for the qubit
    guess_larmor: f64,
    /// Decoherence strength for the qubit
    decoherence: f64,
}

impl QubitArrayBlueprint {
    /// Get a QubitArrayBlueprint object from a Map of Strings to json values. Returns not just the
    /// blueprint but also a vector of parameters to be swept over
    pub fn from_json(
        json_values: Map<String, Value>,
    ) -> (QubitArrayBlueprint, Vec<SweepParameter>) {
        // Values for the first (and only as of now) qubit
        let q1_values: &Map<String, Value> = json_values["q1"].as_object().unwrap();

        // Empty vector for the sweep parameters
        let mut swept_parameters: Vec<SweepParameter> = vec![];

        let (larmor, mut larmor_swept_parameters): (LarmorFrequencyBlueprint, Vec<SweepParameter>) =
            LarmorFrequencyBlueprint::from_json(q1_values["larmor"].as_object().unwrap());

        // Add to the path in the sweep parameter to track it for updates later
        for sweep_parameter in &mut larmor_swept_parameters {
            sweep_parameter.add_path("larmor".to_string());
        }
        // Append the sweep parameters from this gate to the overall
        swept_parameters.append(&mut larmor_swept_parameters);

        // Get the larmor value from the map under the "larmor" key. If it is not a number then
        // assume it is an array and it must be swept over

        // Store the guess lamrmor
        let guess_larmor: f64;

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

        // Decoherence
        let decoherence: f64;

        // Same but for decoherence. If it is not a number it must be an array to sweep over
        if !q1_values["decoherence"].is_number() {
            swept_parameters.push(SweepParameter::from_json(
                "decoherence".to_string(),
                &q1_values["decoherence"],
            ));
            decoherence = swept_parameters[swept_parameters.len() - 1].get_value(0);
        } else {
            decoherence = q1_values["decoherence"].as_f64().unwrap();
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
            "larmor" => self
                .larmor
                .update_parameters(sweep_parameter, path_index + 1, value_index),
            "decoherence" => self.decoherence = sweep_parameter.get_value(value_index),
            _ => return,
        }
        return;
    }
    /// Get a qubit array object constructed from this blueprint
    pub fn get_qubit_array(&self) -> QubitArray {
        return QubitArray::new(
            1,
            self.larmor.get_larmor_frequency(),
            self.guess_larmor,
            self.decoherence,
        );
    }
}
