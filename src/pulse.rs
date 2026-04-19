/// Trait to be implemented to be used as a pulse that makes up a gate
pub trait Pulse {
    /// Get the amplitude of the pulse at some time
    fn get_amplitude(&self, t: f64) -> f64;
    /// Get the frequency of the pulse at some time
    fn get_frequency(&self, t: f64) -> f64;
    /// Get the phase of the pulse at some time
    fn get_phase(&self, t: f64) -> f64;
    /// Get the total duration of the pulse
    fn get_duration(&self) -> f64;
}

/// Struct to hold a pulse which has constant amplitude, frequency, and phase.
pub struct ConstantPulse {
    /// Constant amplitude
    amplitude: f64,
    /// Constant frequency
    frequency: f64,
    /// Constant phase
    phase: f64,
    /// Constant duration
    duration: f64,
}

impl Pulse for ConstantPulse {
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

impl ConstantPulse {
    /// Get a constant pulse object given an amplitude, frequency, phase, and duration
    pub const fn new(amplitude: f64, frequency: f64, phase: f64, duration: f64) -> ConstantPulse {
        return ConstantPulse {
            amplitude: amplitude,
            frequency: frequency,
            phase: phase,
            duration: duration,
        };
    }
}

/// Object to hold a pulse which Ramps any number of amplitude, frequency, and phase over some
/// duration linearly
pub struct RampPulse {
    /// Starting and ending amplitude values
    amplitude_range: (f64, f64),
    /// Starting and ending frequency values
    frequency_range: (f64, f64),
    /// Starting and ending phase values
    phase_range: (f64, f64),
    /// Duration of the pulse
    duration: f64,
}

impl Pulse for RampPulse {
    /// Get the amplitude at some time
    fn get_amplitude(&self, t: f64) -> f64 {
        return (t * ((self.amplitude_range.1 - self.amplitude_range.0) / self.duration))
            + self.amplitude_range.0;
    }
    /// Get the frequency at some time
    fn get_frequency(&self, t: f64) -> f64 {
        return (t * ((self.frequency_range.1 - self.frequency_range.0) / self.duration))
            + self.frequency_range.0;
    }
    /// Get the phase at some time
    fn get_phase(&self, t: f64) -> f64 {
        return (t * ((self.phase_range.1 - self.phase_range.0) / self.duration))
            + self.phase_range.0;
    }
    /// Get the duration of the pulse
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}

impl RampPulse {
    /// Get a RampPulse object given an amplitude, frequency, and phase range with a duration
    pub const fn new(
        amplitude_range: (f64, f64),
        frequency_range: (f64, f64),
        phase_range: (f64, f64),
        duration: f64,
    ) -> RampPulse {
        return RampPulse {
            amplitude_range: amplitude_range,
            frequency_range: frequency_range,
            phase_range: phase_range,
            duration: duration,
        };
    }
}

/// Struct to hold a pulse which is at a constant frequency and phase but which the amplitude of
/// the pulse follows a tangent function. The function is $$A\tan(f(t - o))$$ where A is the
/// coefficient, f is the frequency, and o is the offset.
pub struct TangentPulse {
    /// Coefficient for the tangent pulse
    tan_coefficient: f64,
    /// Frequency of the tangent pulse
    tan_frequency: f64,
    /// Offset in the x direction for the tangent pulse
    tan_offset: f64,
    /// Frequency of the pulse
    frequency: f64,
    /// Phase of the pulse
    phase: f64,
    /// Duration of the pulse
    duration: f64,
}

impl Pulse for TangentPulse {
    /// Get the tangent amplitude of the pulse
    fn get_amplitude(&self, t: f64) -> f64 {
        return self.tan_coefficient * (self.tan_frequency * (t - self.tan_offset)).tan();
    }
    /// Get the frequency of the pulse
    fn get_frequency(&self, _t: f64) -> f64 {
        return self.frequency;
    }
    /// Get the phase of the pulse
    fn get_phase(&self, _t: f64) -> f64 {
        return self.phase;
    }
    /// Get the duration of the pulse
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}

impl TangentPulse {
    /// Get a TangentPulse object given the tangent coefficient, frequency, and offset along wth
    /// the frequency, phase, and duration of the pulse.
    pub const fn new(
        tan_coefficient: f64,
        tan_frequency: f64,
        tan_offset: f64,
        frequency: f64,
        phase: f64,
        duration: f64,
    ) -> TangentPulse {
        return TangentPulse {
            tan_coefficient: tan_coefficient,
            tan_frequency: tan_frequency,
            tan_offset: tan_offset,
            frequency: frequency,
            phase: phase,
            duration: duration,
        };
    }
}
