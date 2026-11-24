use crate::default_name;
use crate::gates::{CheckGateName, Gate};
use crate::pulses::{Constant, Pulse};

/// Struct for a gate that performs a $\pi$ rotation around the +Y axis
pub struct PiY {}

impl PiY {
    /// The gate consists of a single on resonance constant pulse with amplitude of 1 for 1 us
    const PI_Y_PULSE: Constant = Constant::new(1., 0., -(std::f64::consts::PI) / 2., 1.);
    /// Get a box to a PiY gate
    pub fn new() -> Box<PiY> {
        return Box::new(PiY {});
    }
    /// Get a raw PiY object
    pub fn new_raw() -> PiY {
        return PiY {};
    }
}

// Default name is "PiY"
default_name!(PiY);

impl Gate for PiY {
    /// Amplitude from the single Pulse
    fn get_amplitude(&self, t: f64) -> f64 {
        return PiY::PI_Y_PULSE.get_amplitude(t);
    }
    /// Frequency from the single Pulse
    fn get_frequency(&self, t: f64) -> f64 {
        return PiY::PI_Y_PULSE.get_frequency(t);
    }
    /// Phase from the single Pulse
    fn get_phase(&self, t: f64) -> f64 {
        return PiY::PI_Y_PULSE.get_phase(t);
    }
    /// Duration of the single Pulse
    fn get_duration(&self) -> f64 {
        return PiY::PI_Y_PULSE.get_duration();
    }
}
