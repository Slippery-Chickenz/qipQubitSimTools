use crate::{qubit_array::QubitArray, sweep_parameter::{self, SweepParameter}};

use serde::de::value;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct QubitArrayBlueprint {
    larmor: f64,
    guess_larmor: f64,
}

impl QubitArrayBlueprint {
    pub fn from_json(
        json_values: &Map<String, Value>,
    ) -> (QubitArrayBlueprint, Vec<SweepParameter>) {
        let q1_values: &Map<String, Value> = json_values["q1"].as_object().unwrap();

        let mut swept_parameters: Vec<SweepParameter> = vec![];

        let larmor: f64;
        let guess_larmor: f64;

        if q1_values["larmor"].is_array() {
            swept_parameters.push(SweepParameter::new(
                "larmor".to_string(),
                q1_values["larmor"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_f64().unwrap())
                    .collect(),
            ));
            larmor = swept_parameters[swept_parameters.len() - 1].get_value(0);
        } else {
            larmor = q1_values["larmor"].as_f64().unwrap();
        }

        if q1_values["guess_larmor"].is_array() {
            swept_parameters.push(SweepParameter::new(
                "guess_larmor".to_string(),
                q1_values["guess_larmor"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_f64().unwrap())
                    .collect(),
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
    pub fn update_parameters(&mut self, sweep_parameter: &SweepParameter, path_index: usize, value_index: usize) -> () {
        match sweep_parameter.get_path(path_index).as_str() {
            "guess_larmor" => self.guess_larmor = sweep_parameter.get_value(value_index),
            "larmor" => self.larmor = sweep_parameter.get_value(value_index),
            _ => return,
        }
        return;
    }
    pub fn get_qubit_array(&self) -> QubitArray {
        return QubitArray::new(1, self.larmor, self.guess_larmor);
    }
}
