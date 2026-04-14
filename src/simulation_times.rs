use std::fmt;
use std::fmt::Write as _;

use ndarray::Array1;

#[derive(Debug, Clone)]
pub struct UninitializedTimesError;

impl fmt::Display for UninitializedTimesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simulation times are unitialized")
    }
}

pub struct SimulationTimes {
    iteration_times: Vec<Vec<f64>>,
    sample_times: Vec<f64>,
    dt: f64,
}

impl SimulationTimes {
    pub fn new(duration: f64, num_iterations: usize, num_samples: usize) -> SimulationTimes {
        let sample_times: Vec<f64> = Array1::<f64>::linspace(0., duration, num_samples).to_vec();

        let mut iterations_times: Vec<Vec<f64>> = vec![];
        let dt: f64 = duration / (((num_samples - 1) * num_iterations) as f64);
        for i in 0..(num_samples - 1) {
            let next_times: Vec<f64> =
                Array1::<f64>::linspace(sample_times[i], sample_times[i + 1] - dt, num_iterations)
                    .to_vec();
            iterations_times.push(next_times);
        }
        return SimulationTimes {
            iteration_times: iterations_times,
            sample_times: sample_times,
            dt: dt,
        };
    }
    pub fn get_dt(&self) -> f64 {
        return self.dt;
    }
    pub fn get_sample_times(&self) -> &Vec<f64> {
        return &self.sample_times;
    }
    pub fn get_num_samples(&self) -> usize {
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

impl fmt::Display for SimulationTimes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut print_str: String = String::from("");

        print_str.push_str("\nIteration Times:\n---------------------\n");
        for times in &self.iteration_times {
            print_str.push_str("[");
            for el in times {
                write!(&mut print_str, " {},", el)?;
            }
            print_str.pop();
            print_str.push_str("]\n");
        }

        print_str.push_str("--------------------\nSample Times:\n---------------------\n[");
        for el in &self.sample_times {
            write!(&mut print_str, " {},", el)?;
        }
        print_str.pop();
        print_str.push_str("]\n");

        write!(f, "{}", print_str)
    }
}
