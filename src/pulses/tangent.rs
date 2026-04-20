use crate::pulses::Pulse;

/// Struct to hold a pulse which is at a constant frequency and phase but which the amplitude of
/// the pulse follows a tangent function. The function is $$A\tan(f(t - o))$$ where A is the
/// coefficient, f is the frequency, and o is the offset.
pub struct Tangent {
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

impl Pulse for Tangent {
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

impl Tangent {
    /// Get a Tangent object given the tangent coefficient, frequency, and offset along wth
    /// the frequency, phase, and duration of the pulse.
    pub const fn new(
        tan_coefficient: f64,
        tan_frequency: f64,
        tan_offset: f64,
        frequency: f64,
        phase: f64,
        duration: f64,
    ) -> Tangent {
        return Tangent {
            tan_coefficient: tan_coefficient,
            tan_frequency: tan_frequency,
            tan_offset: tan_offset,
            frequency: frequency,
            phase: phase,
            duration: duration,
        };
    }
}
