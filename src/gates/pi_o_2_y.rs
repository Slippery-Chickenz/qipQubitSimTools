use crate::default_name;
use crate::gates::{CheckGateName, Gate};
use crate::pulses::{Constant, Pulse};

/// Struct for a gate that performs a $\pi / 2$ rotation around the +y axis
pub struct PiO2Y {}

// Default name is "PiO2Y"
default_name!(PiO2Y);

impl PiO2Y {
    /// Gate only has a single constant pulse the same as PiO2X but phase shifted by $-\pi/2$
    const PI02Y_PULSE: Constant = Constant::new(1., 0., -(std::f64::consts::PI) / 2., 0.5);
    /// Get a box to a PiO2Y gate
    pub fn new() -> Box<PiO2Y> {
        return Box::new(PiO2Y {});
    }
    /// Get a raw PiO2Y object
    pub fn new_raw() -> PiO2Y {
        return PiO2Y {};
    }
}

impl Gate for PiO2Y {
    /// Amplitude from the single pulse
    fn get_amplitude(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_amplitude(t);
    }
    /// Frequency from the single pulse
    fn get_frequency(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_frequency(t);
    }
    /// Phase from the single pulse
    fn get_phase(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_phase(t);
    }
    /// Duration from the single pulse
    fn get_duration(&self) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_duration();
    }
}
