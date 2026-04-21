use std::fmt;
use std::fmt::Write as _;

use ndarray::Array1;

/// Error returned if a function is called which needs simulation times that are not set
#[derive(Debug, Clone)]
pub(super) struct UninitializedTimesError;

impl fmt::Display for UninitializedTimesError {
    /// Error message for uninitialized times
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simulation times are unitialized")
    }
}

/// Struct to hold the times for a simulation. The number of iterations per sample and the number
/// of samples along with the dt per time step. If the number of samples is 4 and the number of
/// iterations is 25 then there would be 100 total time steps
pub struct SimulationTimes {
    /// Number of iterations per sample
    iteration_times: Vec<Vec<f64>>,
    /// Number of samples to save
    sample_times: Vec<f64>,
    /// Time difference between iterations
    dt: f64,
}

impl SimulationTimes {
    /// Make a new SimulationTimes object given a duration, number of iterations per sample and
    /// the number of samples
    pub fn new(duration: f64, num_iterations: usize, num_samples: usize) -> SimulationTimes {
        // Sample times will be equally spaced throughout the duration
        let mut sample_times: Vec<f64> = Array1::<f64>::linspace(0., duration, num_samples.max(2)).to_vec();

        // Iteration times should be a 2d vector. First axis is the sample number and the second is
        // the iteration within that sample
        let mut iterations_times: Vec<Vec<f64>> = vec![];
        // Calculate the dt for the times
        let dt: f64 = duration / (((sample_times.len() - 1) * num_iterations) as f64);
        // Loop over the number of samples to fill in the iteration times
        for i in 0..(sample_times.len() - 1) {
            // Next set of times is the number of iteratiosn between the current sample time and
            // the next one. The final time is without dt because the iterations are time steps not
            // time stamps
            let next_times: Vec<f64> =
                Array1::<f64>::linspace(sample_times[i], sample_times[i + 1] - dt, num_iterations)
                    .to_vec();
            iterations_times.push(next_times);
        }

        // If there is only supposed to be one sample then remove the starting sample
        if num_samples == 1 {
            sample_times.swap_remove(0);
        }

        return SimulationTimes {
            iteration_times: iterations_times,
            sample_times: sample_times,
            dt: dt,
        };
    }
    /// Get the dt for each time step
    pub fn get_dt(&self) -> f64 {
        return self.dt;
    }
    /// Get the times that each sample are taken at
    pub fn get_sample_times(&self) -> &Vec<f64> {
        return &self.sample_times;
    }
    /// Get the number of samples that are saved
    pub fn get_num_samples(&self) -> usize {
        return self.sample_times.len().max(2);
    }
    /// Get the number of iterations between each sample
    pub fn get_num_iterations_per_sample(&self) -> usize {
        return self.iteration_times[0].len();
    }
    /// Get all the iteration times
    pub fn get_iteration_times(&self) -> &Vec<Vec<f64>> {
        return &self.iteration_times;
    }
    /// Get the iteration times after a specific sample number
    pub fn get_iteration_times_after_sample(&self, sample_num: usize) -> &Vec<f64> {
        return &(self.iteration_times[sample_num]);
    }
}

impl fmt::Display for SimulationTimes {
    /// Pretty display for the sample times
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
