use crate::pulses::Pulse;

/// Struct to hold a pulse which has constant amplitude, frequency, and phase.
pub struct Constant {
    /// Constant amplitude
    amplitude: f64,
    /// Constant frequency
    frequency: f64,
    /// Constant phase
    phase: f64,
    /// Constant duration
    duration: f64,
}

impl Pulse for Constant {
    /// Get the amplitude of the constant pulse
    fn get_amplitude(&self, _t: f64) -> f64 {
        return self.amplitude;
    }
    /// Get the frequency of the constant pulse
    fn get_frequency(&self, _t: f64) -> f64 {
        return self.frequency;
    }
    /// Get the phase of the constant pulse
    fn get_phase(&self, _t: f64) -> f64 {
        return self.phase;
    }
    /// Get the duration of the constant pulse
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}

impl Constant {
    /// Get a constant pulse object given an amplitude, frequency, phase, and duration
    pub const fn new(amplitude: f64, frequency: f64, phase: f64, duration: f64) -> Constant {
        return Constant {
            amplitude: amplitude,
            frequency: frequency,
            phase: phase,
            duration: duration,
        };
    }
}
