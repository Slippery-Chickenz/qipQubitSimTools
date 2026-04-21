use crate::default_name;
use crate::gates::{CheckGateName, Gate};
use crate::pulses::{Constant, Pulse};

/// Struct for a gate that performs a $\pi/2$ rotation around the +x axis
pub struct PiO2X {}

impl PiO2X {
    /// The gate consists of a single on resonance constant pulse with amplitude of 1 for 0.5 us
    const PI02X_PULSE: Constant = Constant::new(1., 0., 0., 0.5);
    /// Get a box to a PiO2X gate
    pub fn new() -> Box<PiO2X> {
        return Box::new(PiO2X {});
    }
    /// Get a raw PiO2X object
    pub fn new_raw() -> PiO2X {
        return PiO2X {};
    }
}

// Default name is "PiO2X"
default_name!(PiO2X);

impl Gate for PiO2X {
    /// Amplitude from the single Pulse
    fn get_amplitude(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_amplitude(t);
    }
    /// Frequency from the single Pulse
    fn get_frequency(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_frequency(t);
    }
    /// Phase from the single Pulse
    fn get_phase(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_phase(t);
    }
    /// Duration of the single Pulse
    fn get_duration(&self) -> f64 {
        return PiO2X::PI02X_PULSE.get_duration();
    }
}
