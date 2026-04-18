pub trait Pulse {
    fn get_amplitude(&self, t: f64) -> f64;
    fn get_frequency(&self, t: f64) -> f64;
    fn get_phase(&self, t: f64) -> f64;
    fn get_duration(&self) -> f64;
}

pub struct ConstantPulse {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    duration: f64,
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

pub struct RampPulse {
    amplitude_range: (f64, f64),
    frequency_range: (f64, f64),
    phase_range: (f64, f64),
    duration: f64,
}


impl Pulse for RampPulse {
    fn get_amplitude(&self, t: f64) -> f64 {
        return (t * ((self.amplitude_range.1 - self.amplitude_range.0) / self.duration)) + self.amplitude_range.0;
    }
    fn get_frequency(&self, t: f64) -> f64 {
        return (t * ((self.frequency_range.1 - self.frequency_range.0) / self.duration)) + self.frequency_range.0;
    }
    fn get_phase(&self, t: f64) -> f64 {
        return (t * ((self.phase_range.1 - self.phase_range.0) / self.duration)) + self.phase_range.0;
    }
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}

impl RampPulse {
    pub const fn new(amplitude_range: (f64, f64), frequency_range: (f64, f64), phase_range: (f64, f64), duration: f64) -> RampPulse {
        return RampPulse {
            amplitude_range: amplitude_range,
            frequency_range: frequency_range,
            phase_range: phase_range,
            duration: duration,
        };
    }
}


pub struct TangentPulse {
    tan_coefficient: f64,
    tan_frequency: f64,
    tan_offset: f64,
    frequency: f64,
    phase: f64,
    duration: f64,
}


impl Pulse for TangentPulse {
    fn get_amplitude(&self, t: f64) -> f64 {
        return self.tan_coefficient * (self.tan_frequency * (t - self.tan_offset)).tan();
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

impl TangentPulse {
    pub const fn new(tan_coefficient: f64, tan_frequency: f64, tan_offset: f64, frequency: f64, phase: f64, duration: f64) -> TangentPulse {
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

