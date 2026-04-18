use serde_json::{Value, Map};
use ndarray::Array1;

#[derive(Debug)]
pub struct SweepParameter {
    path: Vec<String>,
    values: Vec<f64>,
}

impl SweepParameter {
    pub fn new(path: String, values: Vec<f64>) -> SweepParameter {
        return SweepParameter {
            path: vec![path],
            values: values,
        };
    }
    pub fn from_json(path: String, values: &Value) -> SweepParameter {

        if values.is_array() {
            return SweepParameter { path: vec![path], values: values.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect() };
        }

        let values_map: &Map<String, Value> = values.as_object().unwrap();

        if values_map.contains_key("linspace") {
            let linspace_args: Vec<f64> = values_map["linspace"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
            return SweepParameter { path: vec![path], values: Array1::<f64>::linspace(linspace_args[0], linspace_args[1], linspace_args[2] as usize ).to_vec() };
        }
        return SweepParameter { path: vec![path], values: vec![] }
    }
    pub fn add_path(&mut self, path: String) -> () {
        self.path.push(path);
        return;
    }
    pub fn get_path(&self, index: usize) -> &String {
        return &self.path[index];
    }
    pub fn get_full_path(&self) -> String {
        let mut full_path: String = String::new();

        for path_string in &self.path {
            full_path.push_str(&path_string);
            full_path.push_str("_");
        }
        full_path.pop();
        return full_path;
    }
    pub fn get_value(&self, index: usize) -> f64 {
        return self.values[index];
    }
    pub fn get_values(&self) -> &Vec<f64> {
        return &self.values;
    }
    pub fn reverse_path(&mut self) -> () {
        self.path.reverse();
        return;
    }
    pub fn values_len(&self) -> usize {
        return self.values.len();
    }
}

pub trait ParameterUpdate {
    fn update_parameter(
        &mut self,
        sweep_parameter: &SweepParameter,
        path_index: usize,
        value_index: usize,
    ) -> ();
}
