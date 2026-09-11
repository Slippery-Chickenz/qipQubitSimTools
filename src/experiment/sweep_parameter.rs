use ndarray::Array1;
use rand::RngExt;
use rand::distr::Uniform;
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
    /// ID value for this parameter. Parameters with the same ID must have the same length and will
    /// be swept across together thus using the same axis in the final results.
    _id: Option<i64>,
}

impl SweepParameter {
    pub fn new(path: Vec<String>, values: Vec<f64>, id: Option<i64>) -> SweepParameter {
        return SweepParameter {
            path: path,
            values: values,
            _id: id,
        };
    }
    /// Get a sweep parameter from a starting path and json values.
    pub fn from_json(path: &str, json_values: Value) -> (f64, Option<SweepParameter>) {
        if json_values.is_number() {
            return (serde_json::from_value(json_values).unwrap(), Option::None);
        }

        // Values to sweep the parameter over
        let sweep_values: Vec<f64>;

        // If the values are an array then just return a sweep parameter with the path and the
        // values converted to an array of f64s
        if json_values.is_array() {
            sweep_values = serde_json::from_value(json_values).unwrap();
            return (
                sweep_values[0],
                Some(SweepParameter {
                    path: vec![path.to_string()],
                    values: sweep_values,
                    _id: Option::None,
                }),
            );
        }

        // Otherwise the values should be a map from String to values
        let mut values_map: Map<String, Value> = serde_json::from_value(json_values).unwrap();

        // If the values contain an ID then grab that
        let id: Option<i64> = if values_map.contains_key("id") {
            Option::Some(values_map["id"].as_i64().unwrap())
        } else {
            Option::None
        };

        if values_map.contains_key("values") {
            sweep_values = serde_json::from_value(values_map.remove("values").unwrap()).unwrap();
        }
        // If a linspace is defined
        else if values_map.contains_key("linspace") {
            // If it is a linspace then just return a sweep parameter and create the array of
            // values with the ndarray linspace function
            let linspace_args: Vec<f64> =
                serde_json::from_value(values_map.remove("linspace").unwrap()).unwrap();
            sweep_values = Array1::<f64>::linspace(
                linspace_args[0],
                linspace_args[1],
                linspace_args[2] as usize,
            )
            .to_vec();
        }
        // Or define a type of random distribution to sample from
        else if values_map.contains_key("distribution") {
            let distribution: &Map<String, Value> = values_map["distribution"].as_object().unwrap();
            // Uniform distribution
            if distribution["type"].as_str().unwrap() == "uniform" {
                let min: f64 = distribution["min"].as_f64().unwrap();
                let max: f64 = distribution["max"].as_f64().unwrap();
                let distr: Uniform<f64> = Uniform::<f64>::try_from(min..max).unwrap();
                let rng = rand::rng();
                sweep_values = rng
                    .sample_iter(distr)
                    .take(distribution["num_samples"].as_i64().unwrap() as usize)
                    .collect();
            } else {
                panic!("No valid values given for sweep {}", path);
            }
        } else {
            panic!("No valid values given for sweep {}", path);
        }

        return (
            sweep_values[0],
            Some(SweepParameter {
                path: vec![path.to_string()],
                values: sweep_values,
                _id: id,
            }),
        );
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
