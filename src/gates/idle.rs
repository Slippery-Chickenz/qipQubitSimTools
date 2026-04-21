use crate::default_name;
use crate::gates::{CheckGateName, Gate};

/// Struct for a gate which just idles
pub struct Idle {
    /// Duration to idle
    duration: f64,
}

impl Idle {
    /// Get a Box to an Idle gate given a duration
    pub fn new(duration: f64) -> Box<Idle> {
        return Box::new(Idle { duration });
    }
    /// Get a raw Idle gate object given a duration
    pub fn new_raw(duration: f64) -> Idle {
        return Idle { duration };
    }
}

// Default name for the idle gate
default_name!(Idle);

impl Gate for Idle {
    /// Amplitude for idle is 0
    fn get_amplitude(&self, _t: f64) -> f64 {
        return 0.;
    }
    /// Frequency for idle is 0
    fn get_frequency(&self, _t: f64) -> f64 {
        return 0.;
    }
    /// Phase for idle is 0
    fn get_phase(&self, _t: f64) -> f64 {
        return 0.;
    }
    /// Get the duration for the idle gate
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}
