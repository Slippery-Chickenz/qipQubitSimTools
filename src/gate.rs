use crate::pulse::{ConstantPulse, Pulse, TangentPulse, RampPulse};

use rootfinder::{root_bisection, Interval};

pub trait CheckGateName {
    fn check_name(name: &str) -> bool;
}

pub trait Gate {
    fn get_amplitude(&self, t: f64) -> f64;
    fn get_frequency(&self, t: f64) -> f64;
    fn get_phase(&self, t: f64) -> f64;
    fn get_duration(&self) -> f64;
}

macro_rules! default_name {
    ($n:ident) => {
        impl CheckGateName for $n {
            fn check_name(name: &str) -> bool {
                return name == stringify!($n);
            }
        }
    };
}

pub struct Idle {
    duration: f64,
}

impl Idle {
    pub fn new(duration: f64) -> Box<Idle> {
        return Box::new(Idle { duration });
    }
    pub fn new_raw(duration: f64) -> Idle {
        return Idle { duration };
    }
}

default_name!(Idle);

impl Gate for Idle {
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
    pub fn new() -> Box<PiO2X> {
        return Box::new(PiO2X {});
    }
    pub fn new_raw() -> PiO2X {
        return PiO2X {};
    }
}

default_name!(PiO2X);

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

default_name!(PiO2Y);

impl PiO2Y {
    const PI02Y_PULSE: ConstantPulse =
        ConstantPulse::new(1., 0., -(std::f64::consts::PI) / 2., 0.5);
    pub fn new() -> Box<PiO2Y> {
        return Box::new(PiO2Y {});
    }
    pub fn new_raw() -> PiO2Y {
        return PiO2Y {};
    }
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

pub struct ATMGate {
    pulses: Vec<Box<dyn Pulse>>,
    duration: f64,
}
default_name!(ATMGate);

impl ATMGate {
    pub fn new_raw(
        rise_time: f64, 
        fall_time: f64,
        max_amplitude: f64,
        max_frequency: f64,
        rise_gradient_percent: f64,
        fall_gradient_percent: f64,
        duration: f64,
    ) ->ATMGate {

        let rise_gradient: f64 = max_amplitude * rise_gradient_percent / rise_time;
        let fall_gradient: f64 = max_amplitude * fall_gradient_percent / fall_time;

        let mut pulses: Vec<Box<dyn Pulse>> = vec![];

        let rise_b: f64 = ATMGate::find_beta(rise_time, rise_gradient, max_amplitude);
        let rise_a: f64 = rise_gradient / rise_b;
        let fall_b: f64 = ATMGate::find_beta(-fall_time, -fall_gradient, max_amplitude);
        let fall_a: f64 = fall_gradient / fall_b;
        pulses.push(Box::new(TangentPulse::new(rise_a, rise_b, 0., max_frequency, 0., rise_time)));
        // pulses.push(Box::new(ConstantPulse::new(max_amplitude, max_frequency, 0., rise_time)));
        pulses.push(Box::new(RampPulse::new((max_amplitude, max_amplitude),
                                   (max_frequency, 0.), 
                                   (0., 0.), duration - rise_time - fall_time,)));
        pulses.push(Box::new(TangentPulse::new(-fall_a, fall_b, fall_time, 0., 0., fall_time)));

        return ATMGate { pulses: pulses, duration: duration};
    }
    fn find_beta(rise_time: f64, rise_gradient: f64, max_amplitude: f64) -> f64 {

        let lower_bound_b: f64 = 1e-10;
        let upper_bound_b: f64 = 15.;

        let root_to_find = |b: f64| ((b * max_amplitude)/rise_gradient).atan() - b*rise_time;
        if root_to_find(lower_bound_b) * root_to_find(upper_bound_b) > 0. {
            panic!("Bad root finding");
        }
        let sol: f64 = root_bisection(&root_to_find, Interval::new(1e-10, 15.), None, None).unwrap();
        return sol;
    }
    fn get_pulse_index(&self, mut time: f64) -> (usize, f64) {
        for (i, gate) in self.pulses.iter().enumerate() {
            let gate_duration: f64 = gate.get_duration();
            if time <= gate_duration {
                return (i, time);
            }
            time -= gate_duration;
        }
        panic!("Tried to get gate index for time past the durration of the circuit.")
    }
}

impl Gate for ATMGate {
    fn get_amplitude(&self, t: f64) -> f64 {
        let (i, time) = self.get_pulse_index(t);
        return self.pulses[i].get_amplitude(time);
    }
    fn get_frequency(&self, t: f64) -> f64 {
        let (i, time) = self.get_pulse_index(t);
        return self.pulses[i].get_frequency(time);
    }
    fn get_phase(&self, t: f64) -> f64 {
        let (i, time) = self.get_pulse_index(t);
        return self.pulses[i].get_phase(time);
    }
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}


