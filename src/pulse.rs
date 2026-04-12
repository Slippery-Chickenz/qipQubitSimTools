pub struct ConstantPulse {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    duration: f64,
}

pub trait Pulse {
    fn get_amplitude(&self, t: f64) -> f64;
    fn get_frequency(&self, t: f64) -> f64;
    fn get_phase(&self, t: f64) -> f64;
    fn get_duration(&self) -> f64;
}

impl Pulse for ConstantPulse {
    fn get_amplitude(&self, _t: f64) -> f64 {
        return self.amplitude;
    }
    fn get_frequency(&self, _t: f64) -> f64 {
        return self.frequency;
    }
    fn get_phase(&self, _t: f64) -> f64 {
        return self.phase;
    }
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}

impl ConstantPulse {
    pub const fn new(amplitude: f64, frequency: f64, phase: f64, duration: f64) -> ConstantPulse {
        return ConstantPulse {
            amplitude: amplitude,
            frequency: frequency,
            phase: phase,
            duration: duration,
        };
    }
}
