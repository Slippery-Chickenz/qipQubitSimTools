use ndarray::Array1;
use serde_json::{Map, Value};

/// Hold a parameter to sweep across in an experiment. Has to hold both the path to update the
/// parameter and the values to sweep over.
#[derive(Debug)]
pub struct SweepParameter {
    /// Path to follow to update the parameter. Each blueprint that takes in a json should add to
    /// this path so when it is called to update the parameter it can find its point and forward
    /// the update correctly
    path: Vec<String>,
    /// Values to sweep the parameter over
    values: Vec<f64>,
}

impl SweepParameter {
    /// Get a sweep parameter from a starting path and json values.
    pub fn from_json(path: String, values: &Value) -> SweepParameter {
        // If the values are an array then just return a sweep parameter with the path and the
        // values converted to an array of f64s
        if values.is_array() {
            return SweepParameter {
                path: vec![path],
                values: values
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_f64().unwrap())
                    .collect(),
            };
        }

        // Otherwise the values should be a map from String to values
        let values_map: &Map<String, Value> = values.as_object().unwrap();

        // Currently we support listing the sweep values as a linspace from a min to a max with
        // some number of values
        if values_map.contains_key("linspace") {
            // If it is a linspace then just return a sweep parameter and create the array of
            // values with the ndarray linspace function
            let linspace_args: Vec<f64> = values_map["linspace"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_f64().unwrap())
                .collect();
            return SweepParameter {
                path: vec![path],
                values: Array1::<f64>::linspace(
                    linspace_args[0],
                    linspace_args[1],
                    linspace_args[2] as usize,
                )
                .to_vec(),
            };
        }
        return SweepParameter {
            path: vec![path],
            values: vec![],
        };
    }
    /// Add a string onto the end of the path values
    pub fn add_path(&mut self, path: String) -> () {
        self.path.push(path);
        return;
    }
    /// Get the path value at a certain index
    pub fn get_path(&self, index: usize) -> &String {
        return &self.path[index];
    }
    /// Get the full path as a string
    pub fn get_full_path(&self) -> String {
        // Just construct a new string and append the path strings onto it separated by an _
        let mut full_path: String = String::new();
        for path_string in &self.path {
            full_path.push_str(&path_string);
            full_path.push_str("_");
        }
        full_path.pop(); // Remove extra _
        return full_path;
    }
    /// The value of a parameter to use at a certain index
    pub fn get_value(&self, index: usize) -> f64 {
        return self.values[index];
    }
    /// Get all the values that are being swept over
    pub fn get_values(&self) -> &Vec<f64> {
        return &self.values;
    }
    /// Reverse the path so it can be read from front to back. Normally when constructed it is
    /// returned back through the functions that it would take to update it so when paths are added
    /// they are usuall added onto the end it the reverse order
    pub fn reverse_path(&mut self) -> () {
        self.path.reverse();
        return;
    }
    /// Get the length of the values to sweep over
    pub fn values_len(&self) -> usize {
        return self.values.len();
    }
}
