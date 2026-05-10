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

/// Struct to hold the times for a simulation and the indices of the time values at which the
/// samples should be saved.
pub struct SimulationTimes {
    /// Time values at each iteration
    iteration_times: Array2<f64>,
    /// Indicies of the samples in the iteration times vector
    sample_indices: Vec<usize>,
    /// Time difference between iterations
    dt: f64,
}

impl SimulationTimes {
    /// Make a new SimulationTimes object given a duration, step size and number of samples to save
    pub fn new(duration: f64, step_size: f64, num_samples: usize) -> SimulationTimes {
        // Iteration times without sub timings for fourth order runge kutta
        let temp_iteration_times: Array1<f64> = Array1::<f64>::range(0., duration, step_size);

        // Set the iteration times based on the step size and duration of the simulation
        let mut iteration_times: Array2<f64> =
            Array2::<f64>::zeros([temp_iteration_times.len(), 4]);
        iteration_times.column_mut(0).assign(&temp_iteration_times);

        // Indices in the iteration times for each sample to be saved at
        let mut sample_indicies: Vec<usize> = vec![];

        if num_samples != 1 {
            let sample_index_spacing: usize = iteration_times.shape()[0] / (num_samples - 1);
            for i in 0..num_samples - 1 {
                sample_indicies.push(i * sample_index_spacing);
            }
        }

        sample_indicies.push(iteration_times.shape()[0]);
        return SimulationTimes {
            iteration_times: iteration_times,
            sample_indices: sample_indicies,
            dt: step_size,
        };
    }
    /// Get the dt for each time step
    pub fn get_dt(&self) -> f64 {
        return self.dt;
    }
    /// Get the indices where each sample is saved
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
