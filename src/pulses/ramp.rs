use crate::pulses::Pulse;

/// Object to hold a pulse which Ramps any number of amplitude, frequency, and phase over some
/// duration linearly
pub struct Ramp {
    /// Starting and ending amplitude values
    amplitude_range: (f64, f64),
    /// Starting and ending frequency values
    frequency_range: (f64, f64),
    /// Starting and ending phase values
    phase_range: (f64, f64),
    /// Duration of the pulse
    duration: f64,
}

impl Pulse for Ramp {
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

impl Ramp {
    /// Get a Ramp object given an amplitude, frequency, and phase range with a duration
    pub const fn new(
        amplitude_range: (f64, f64),
        frequency_range: (f64, f64),
        phase_range: (f64, f64),
        duration: f64,
    ) -> Ramp {
        return Ramp {
            amplitude_range: amplitude_range,
            frequency_range: frequency_range,
            phase_range: phase_range,
            duration: duration,
        };
    }
}
