use ndarray::Array1;
use num_complex::Complex64;
use serde_json::Value;

pub fn get_state_from_json(json_value: Value) -> Array1<Complex64> {
    // Check for the inital state and assign it depending on input
    if json_value.is_string() {
        return match json_value.as_str().unwrap() {
            "+x" => Array1::<Complex64>::from_vec(vec![
                Complex64::new(1. / 2_f64.sqrt(), 0.),
                Complex64::new(1. / 2_f64.sqrt(), 0.),
            ]),
            "-x" => Array1::<Complex64>::from_vec(vec![
                Complex64::new(1. / 2_f64.sqrt(), 0.),
                Complex64::new(-1. / 2_f64.sqrt(), 0.),
            ]),
            "+y" => Array1::<Complex64>::from_vec(vec![
                Complex64::new(1. / 2_f64.sqrt(), 0.),
                Complex64::new(0., 1. / 2_f64.sqrt()),
            ]),
            "-y" => Array1::<Complex64>::from_vec(vec![
                Complex64::new(1. / 2_f64.sqrt(), 0.),
                Complex64::new(0., -1. / 2_f64.sqrt()),
            ]),
            "+z" => {
                Array1::<Complex64>::from_vec(vec![Complex64::new(1., 0.), Complex64::new(0., 0.)])
            }
            "-z" => {
                Array1::<Complex64>::from_vec(vec![Complex64::new(0., 0.), Complex64::new(1., 0.)])
            }
            _ => panic!("Not valid inital state string"),
        };
    } else if json_value.is_object() {
        return Array1::<Complex64>::from_vec(vec![
            Complex64::new(
                json_value["init_state"].as_object().unwrap()["+z"]
                    .as_object()
                    .unwrap()["real"]
                    .as_f64()
                    .unwrap(),
                json_value["init_state"].as_object().unwrap()["+z"]
                    .as_object()
                    .unwrap()["imag"]
                    .as_f64()
                    .unwrap(),
            ),
            Complex64::new(
                json_value["init_state"].as_object().unwrap()["-z"]
                    .as_object()
                    .unwrap()["real"]
                    .as_f64()
                    .unwrap(),
                json_value["init_state"].as_object().unwrap()["-z"]
                    .as_object()
                    .unwrap()["imag"]
                    .as_f64()
                    .unwrap(),
            ),
        ]);
    } else if json_value.is_boolean() {
        if json_value.as_bool().unwrap() {
            return Array1::<Complex64>::from_vec(vec![
                Complex64::new(1., 0.),
                Complex64::new(0., 0.),
            ]);
        }
    }
    panic!("Not value json for a QubitState!");
}
