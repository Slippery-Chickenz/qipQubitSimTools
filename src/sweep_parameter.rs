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
    pub fn add_path(&mut self, path: String) -> () {
        self.path.push(path);
        return;
    }
    pub fn get_path(&self, index: usize) -> &String {
        return &self.path[index];
    }
    pub fn get_value(&self, index: usize) -> f64 {
        return self.values[index];
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
