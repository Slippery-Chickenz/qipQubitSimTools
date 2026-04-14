use crate::qubit_array::QubitArray;

use ndarray::Array2;
use num_complex::Complex64;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct QubitArrayBlueprint {
    larmor: f64,
    guess_larmor: f64,
}

impl QubitArrayBlueprint {
    pub fn from_json(json_values: &Map<String, Value>) -> QubitArrayBlueprint {
        let q1_values: &Map<String, Value> = json_values["q1"].as_object().unwrap();
        return QubitArrayBlueprint {
            larmor: q1_values["larmor"].as_f64().unwrap(),
            guess_larmor: q1_values["guess_larmor"].as_f64().unwrap(),
        };
    }
    pub fn get_qubit_array(&self) -> QubitArray {

        return QubitArray::new(1, 0., 0.);
    }
}
