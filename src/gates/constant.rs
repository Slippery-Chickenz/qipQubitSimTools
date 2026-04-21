use crate::default_name;
use crate::gates::{CheckGateName, Gate};
use crate::pulses::{self, Pulse};

/// Struct for a gate which just idles
pub struct Constant {
    /// Constant Pulse to apply for some duration
    constant_pulse: pulses::Constant,
}

impl Constant {
    /// Get a Box to an Idle gate given a duration
    pub fn new(amplitude: f64, frequency: f64, phase: f64, duration: f64) -> Box<Constant> {
        return Box::new(Constant {
            constant_pulse: pulses::Constant::new(amplitude, frequency, phase, duration),
        });
    }
    /// Get a raw Idle gate object given a duration
    pub fn new_raw(amplitude: f64, frequency: f64, phase: f64, duration: f64) -> Constant {
        return Constant {
            constant_pulse: pulses::Constant::new(amplitude, frequency, phase, duration),
        };
    }
}

// Default name for the idle gate
default_name!(Constant);

impl Gate for Constant {
    /// Amplitude for idle is 0
    fn get_amplitude(&self, t: f64) -> f64 {
        return self.constant_pulse.get_amplitude(t);
    }
    /// Frequency for idle is 0
    fn get_frequency(&self, t: f64) -> f64 {
        return self.constant_pulse.get_frequency(t);
    }
    /// Phase for idle is 0
    fn get_phase(&self, t: f64) -> f64 {
        return self.constant_pulse.get_phase(t);
    }
    /// Get the duration for the idle gate
    fn get_duration(&self) -> f64 {
        return self.constant_pulse.get_duration();
    }
}
