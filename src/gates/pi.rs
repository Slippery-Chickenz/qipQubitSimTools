use crate::default_name;
use crate::gates::{CheckGateName, Gate};

/// Struct for a gate that performs a $\pi$ rotation around the +z axis
pub struct Pi {}

impl Pi {
    /// Get a box to a Pi gate
    pub fn new() -> Box<Pi> {
        return Box::new(Pi {});
    }
    /// Get a raw Pi object
    pub fn new_raw() -> Pi {
        return Pi {};
    }
}

// Default name is "PiO2X"
default_name!(Pi);

impl Gate for Pi {
    /// Amplitude from the single Pulse
    fn get_amplitude(&self, _t: f64) -> f64 {
        return 0.;
    }
    /// Frequency from the single Pulse
    fn get_frequency(&self, _t: f64) -> f64 {
        return 0.;
    }
    /// Phase from the single Pulse
    fn get_phase(&self, _t: f64) -> f64 {
        return 0.;
    }
    /// Duration of the single Pulse
    fn get_duration(&self) -> f64 {
        return 0.;
    }
}
