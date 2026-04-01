mod pulse;

trait Gate {
    fn get_amplitude(&self, t: f64) -> f64;
    fn get_frequency(&self, t: f64) -> f64;
    fn get_phase(&self, t: f64) -> f64;
    fn get_length(&self) -> f64;
}


struct IdleGate {
    length: f64,
}

impl Gate for IdleGate {
    fn get_amplitude(&self, _t: f64) -> f64 {
        return 0.
    }
    fn get_frequency(&self, _t: f64) -> f64 {
        return 0.
    }
    fn get_phase(&self, _t: f64) -> f64 {
        return 0.
    }
    fn get_length(&self) -> f64 {
        return self.length
    }
}


struct PiO2X {}

impl PiO2X {
    const PI02X_PULSE: pulse::ConstantPulse = pulse::ConstantPulse::new(1., 0., 0.,0.5);
}

impl Gate for PiO2X {
    fn get_amplitude(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_amplitude(t)
    }
    fn get_frequency(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_frequency(t)
    }
    fn get_phase(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_phase(t)
    }
    fn get_length(&self) -> f64 {
        return PiO2X::PI02X_PULSE.get_length()
    }
}


struct PiO2Y {}

impl PiO2Y {
    const PI02Y_PULSE: ConstantPulse = ConstantPulse { amplitude: 1., frequency: 0., phase: (-3.141592 / 2.), length: 0.5 };
}

impl Gate for PiO2Y {
    fn get_amplitude(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_amplitude(t)
    }
    fn get_frequency(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_frequency(t)
    }
    fn get_phase(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_phase(t)
    }
    fn get_length(&self) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_length()
    }
}
