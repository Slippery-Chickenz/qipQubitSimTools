use crate::pulse::ConstantPulse;
use crate::pulse::Pulse;

pub trait Gate {
    fn get_amplitude(&self, t: f64) -> f64;
    fn get_frequency(&self, t: f64) -> f64;
    fn get_phase(&self, t: f64) -> f64;
    fn get_duration(&self) -> f64;
}

pub struct IdleGate {
    duration: f64,
}

impl IdleGate {
    pub fn new(duration: f64) -> IdleGate {
        return IdleGate { duration };
    }
}

impl Gate for IdleGate {
    fn get_amplitude(&self, _t: f64) -> f64 {
        return 0.;
    }
    fn get_frequency(&self, _t: f64) -> f64 {
        return 0.;
    }
    fn get_phase(&self, _t: f64) -> f64 {
        return 0.;
    }
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}

pub struct PiO2X {}

impl PiO2X {
    const PI02X_PULSE: ConstantPulse = ConstantPulse::new(1., 0., 0., 0.5);
    pub fn new() -> PiO2X {
        return PiO2X {};
    }
}

impl Gate for PiO2X {
    fn get_amplitude(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_amplitude(t);
    }
    fn get_frequency(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_frequency(t);
    }
    fn get_phase(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_phase(t);
    }
    fn get_duration(&self) -> f64 {
        return PiO2X::PI02X_PULSE.get_duration();
    }
}

pub struct PiO2Y {}

impl PiO2Y {
    const PI02Y_PULSE: ConstantPulse =
        ConstantPulse::new(1., 0., -(std::f64::consts::PI) / 2., 0.5);
}

impl Gate for PiO2Y {
    fn get_amplitude(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_amplitude(t);
    }
    fn get_frequency(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_frequency(t);
    }
    fn get_phase(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_phase(t);
    }
    fn get_duration(&self) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_duration();
    }
}
