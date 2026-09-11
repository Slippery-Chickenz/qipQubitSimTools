// use crate::blueprints::LarmorFrequencyBlueprint;
use crate::simulation::QubitArray;
use crate::utils::get_state_from_json;
use crate::{blueprints::LarmorFrequencyBlueprint, experiment::SweepParameter};

use ndarray::Array1;
use num_complex::Complex64;
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
    /// Inital state for the qubit
    init_state: Array1<Complex64>,
}

impl QubitArrayBlueprint {
    /// Get a QubitArrayBlueprint object from a Map of Strings to json values. Returns not just the
    /// blueprint but also a vector of parameters to be swept over
    pub fn from_json(mut json_values: Vec<Value>) -> (QubitArrayBlueprint, Vec<SweepParameter>) {
        // Values for the first (and only as of now) qubit
        let mut q1_values: Map<String, Value> =
            serde_json::from_value(json_values.remove(0)).unwrap();

        // Empty vector for the sweep parameters
        let mut swept_parameters: Vec<SweepParameter> = vec![];

        let (larmor, mut larmor_swept_parameters): (LarmorFrequencyBlueprint, Vec<SweepParameter>) =
            LarmorFrequencyBlueprint::from_json(
                serde_json::from_value(q1_values.remove("larmor").unwrap()).unwrap(),
            );

        // Add to the path in the sweep parameter to track it for updates later
        for sweep_parameter in &mut larmor_swept_parameters {
            sweep_parameter.add_path("larmor".to_string());
        }
        // Append the sweep parameters from this gate to the overall
        swept_parameters.append(&mut larmor_swept_parameters);

        let (guess_larmor, sweep_parameter_option): (f64, Option<SweepParameter>) =
            SweepParameter::from_json(
                "guess_larmor",
                serde_json::from_value(q1_values.remove("guess_larmor").unwrap()).unwrap(),
            );
        if let Some(sweep_parameter) = sweep_parameter_option {
            swept_parameters.push(sweep_parameter);
        }

        let (decoherence, sweep_parameter_option): (f64, Option<SweepParameter>) =
            SweepParameter::from_json(
                "decoherence",
                serde_json::from_value(q1_values.remove("decoherence").unwrap()).unwrap(),
            );
        if let Some(sweep_parameter) = sweep_parameter_option {
            swept_parameters.push(sweep_parameter);
        }

        let init_state: Array1<Complex64> =
            get_state_from_json(q1_values.remove("init_state").unwrap());

        return (
            QubitArrayBlueprint {
                larmor: larmor,
                guess_larmor: guess_larmor,
                decoherence: decoherence,
                init_state: init_state,
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
            self.init_state.clone(),
        );
    }
}
