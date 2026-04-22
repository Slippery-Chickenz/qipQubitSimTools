use std::fmt;

use ndarray::{Array1, Array2};

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
    /// Time values at each iteration
    // iteration_times: Vec<Vec<f64>>,
    iteration_times: Array2<f64>,
    /// Number of samples to save
    // sample_times: Vec<f64>,
    /// Indicies of the samples in the iteration times vector
    sample_indices: Vec<usize>,
    /// Time difference between iterations
    dt: f64,
}

impl SimulationTimes {
    /// Make a new SimulationTimes object given a duration, number of iterations per sample and
    /// the number of samples
    // pub fn new(duration: f64, num_iterations: usize, num_samples: usize) -> SimulationTimes {
    pub fn new(duration: f64, step_size: f64, num_samples: usize) -> SimulationTimes {
        // dt for each iteration is the step size
        let dt: f64 = step_size;

        // Iteration times without sub timings for fourth order runge kutta
        let temp_iteration_times: Array1<f64> = Array1::<f64>::range(0., duration, dt);

        // Set the iteration times based on the step size and duration of the simulation
        let mut iteration_times: Array2<f64> =
            Array2::<f64>::zeros([temp_iteration_times.len(), 4]);
        iteration_times.column_mut(0).assign(&temp_iteration_times);

        // Indices in the iteration times for each sample to be saved at
        // let mut sample_times: Array1<f64> = Array1::<f64>::zeros(num_samples);
        let mut sample_indicies: Vec<usize> = vec![];

        if num_samples != 1 {
            let sample_index_spacing: usize = (iteration_times.shape()[0] + (num_samples - 2)) / (num_samples - 1);
            for i in (0..iteration_times.shape()[0]).step_by(sample_index_spacing) {
                sample_indicies.push(i);
            }
        }

        sample_indicies.push(iteration_times.shape()[0]);
        return SimulationTimes {
            iteration_times: iteration_times,
            sample_indices: sample_indicies,
            dt: dt,
        };
    }
    /// Get the dt for each time step
    pub fn get_dt(&self) -> f64 {
        return self.dt;
    }
    /// Get the times that each sample are taken at
    // pub fn get_sample_times(&self) -> &Vec<f64> {
    //     return &self.sample_times;
    // }
    pub fn get_sample_indices(&self) -> &Vec<usize> {
        return &self.sample_indices;
    }
    /// Get the number of samples that are saved
    pub fn get_num_samples(&self) -> usize {
        return self.sample_indices.len();
    }
    /// Get all the iteration times
    pub fn get_iteration_times(&self) -> &Array2<f64> {
        return &self.iteration_times;
    }
    /// Get a specific iteration time based on an index
    pub fn get_iteration_time(&self, index: usize) -> f64 {
        return self.iteration_times[[index, 0]];
    }
    /// Get the number of iterations for the simulation
    pub fn get_num_iterations(&self) -> usize {
        return self.iteration_times.len();
    }
}
