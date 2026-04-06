use ndarray::Array1;

pub struct SimulationTimes {
    iteration_times: Vec<Vec<f64>>,
    sample_times: Vec<f64>,
    sample_indices: Vec<usize>,
}

impl SimulationTimes {
    pub fn new(duration: f64, mut num_iterations: usize, num_samples: usize) -> SimulationTimes {

        // Number of steps rather than number of time values
        num_iterations += 1;

        let sample_times: Vec<f64> = Array1::<f64>::linspace(0., duration, num_samples).to_vec();
        let mut sample_indicies: Vec<usize> = Vec::<usize>::with_capacity(num_samples);

        let mut iterations_times: Vec<Vec<f64>> = vec![Array1::<f64>::linspace(0., sample_times[1], num_iterations).to_vec()];
        iterations_times.pop();

        for i in 0..(num_samples - 2) {
            let mut next_times: Vec<f64> = Array1::<f64>::linspace(sample_times[i + 1], sample_times[i + 2], num_iterations).to_vec();
            if i < num_samples - 3 {
                next_times.pop();
            }
            sample_indicies[i + 1] = iterations_times.len();
            iterations_times.push(next_times);
        }

        return SimulationTimes { iteration_times: iterations_times, sample_times: sample_times, sample_indices: sample_indicies }
    }
    pub fn get_sample_indicies(&self) -> &Vec<usize> {
        return &self.sample_indices;
    }
    pub fn get_num_sample_times(&self) -> usize {
        return self.sample_times.len();
    }
    pub fn get_num_iterations_per_sample(&self) -> usize {
        return self.iteration_times[0].len();
    }
    pub fn get_iteration_times(&self) -> &Vec<Vec<f64>> {
        return &self.iteration_times;
    }
    pub fn get_iteration_times_after_sample(&self, sample_num: usize) -> &Vec<f64> {
        return &(self.iteration_times[sample_num]);
    }
}
