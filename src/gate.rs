use crate::pulse::{ConstantPulse, Pulse, RampPulse, TangentPulse};

use rootfinder::{Interval, root_bisection};

/// Trait to guarentee that a gate can be converted into a string
pub trait CheckGateName {
    fn check_name(name: &str) -> bool;
}

/// Trait that must be implemented for any gate that is to be used within a quantum circuit.
/// Contains functions to get the amplitude, frequency, and phase of the gate at any time within
/// the duration of the gate. And one to get the duration of the gate.
pub trait Gate {
    fn get_amplitude(&self, t: f64) -> f64;
    fn get_frequency(&self, t: f64) -> f64;
    fn get_phase(&self, t: f64) -> f64;
    fn get_duration(&self) -> f64;
}

/// Macro to make the check_name function and implement the CheckGateName trait for an arbitrary
/// struct. By default it just makes the string name the name of the struct
macro_rules! default_name {
    ($n:ident) => {
        impl CheckGateName for $n {
            fn check_name(name: &str) -> bool {
                return name == stringify!($n);
            }
        }
    };
}

/// Struct for a gate which just idles
pub struct Idle {
    /// Duration to idle
    duration: f64,
}

impl Idle {
    /// Get a Box to an Idle gate given a duration
    pub fn new(duration: f64) -> Box<Idle> {
        return Box::new(Idle { duration });
    }
    /// Get a raw Idle gate object given a duration
    pub fn new_raw(duration: f64) -> Idle {
        return Idle { duration };
    }
}

// Default name for the idle gate
default_name!(Idle);

impl Gate for Idle {
    /// Amplitude for idle is 0
    fn get_amplitude(&self, _t: f64) -> f64 {
        return 0.;
    }
    /// Frequency for idle is 0
    fn get_frequency(&self, _t: f64) -> f64 {
        return 0.;
    }
    /// Phase for idle is 0
    fn get_phase(&self, _t: f64) -> f64 {
        return 0.;
    }
    /// Get the duration for the idle gate
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}

/// Struct for a gate that performs a $\pi/2$ rotation around the +x axis
pub struct PiO2X {}

impl PiO2X {
    /// The gate consists of a single on resonance constant pulse with amplitude of 1 for 0.5 us
    const PI02X_PULSE: ConstantPulse = ConstantPulse::new(1., 0., 0., 0.5);
    /// Get a box to a PiO2X gate
    pub fn new() -> Box<PiO2X> {
        return Box::new(PiO2X {});
    }
    /// Get a raw PiO2X object
    pub fn new_raw() -> PiO2X {
        return PiO2X {};
    }
}

// Default name is "PiO2X"
default_name!(PiO2X);

impl Gate for PiO2X {
    /// Amplitude from the single Pulse
    fn get_amplitude(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_amplitude(t);
    }
    /// Frequency from the single Pulse
    fn get_frequency(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_frequency(t);
    }
    /// Phase from the single Pulse
    fn get_phase(&self, t: f64) -> f64 {
        return PiO2X::PI02X_PULSE.get_phase(t);
    }
    /// Duration of the single Pulse
    fn get_duration(&self) -> f64 {
        return PiO2X::PI02X_PULSE.get_duration();
    }
}

/// Struct for a gate that performs a $\pi / 2$ rotation around the +y axis
pub struct PiO2Y {}

// Default name is "PiO2Y"
default_name!(PiO2Y);

impl PiO2Y {
    /// Gate only has a single constant pulse the same as PiO2X but phase shifted by $-\pi/2$
    const PI02Y_PULSE: ConstantPulse =
        ConstantPulse::new(1., 0., -(std::f64::consts::PI) / 2., 0.5);
    /// Get a box to a PiO2Y gate
    pub fn new() -> Box<PiO2Y> {
        return Box::new(PiO2Y {});
    }
    /// Get a raw PiO2Y object
    pub fn new_raw() -> PiO2Y {
        return PiO2Y {};
    }
}

impl Gate for PiO2Y {
    /// Amplitude from the single pulse
    fn get_amplitude(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_amplitude(t);
    }
    /// Frequency from the single pulse
    fn get_frequency(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_frequency(t);
    }
    /// Phase from the single pulse
    fn get_phase(&self, t: f64) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_phase(t);
    }
    /// Duration from the single pulse
    fn get_duration(&self) -> f64 {
        return PiO2Y::PI02Y_PULSE.get_duration();
    }
}

/// Struct for a gate that implements the ATM pulse
pub struct ATMGate {
    /// The ATM Pulse is made up of 3 pulses:
    /// A rising amplitude and constant frequency
    /// A constant amplitude and falling frequency
    /// A falling amplitude and constant frequency
    pulses: Vec<Box<dyn Pulse>>,
    // Duration for the total pulse
    duration: f64,
}

// Default name is "ATMGate"
default_name!(ATMGate);

impl ATMGate {
    /// Get a new raw ATMGate object given the gate parameters:
    ///
    /// max_amplitude: Maximum amplitude to rise to
    /// max_frequency: Maximum frequency detuning to start at
    /// rise_gradient_percent: Value from (0, 1) that defines how steep the rising pulse starts at
    /// fall_gradient_percent: Value from (0, 1) that defines how steep the falling pulse ends at
    /// rise_duration: Duration of the rising pulse
    /// fall_duration: Duration of the falling pulse
    /// duration: Total duration of the pulse
    pub fn new_raw(
        max_amplitude: f64,
        max_frequency: f64,
        rise_gradient_percent: f64,
        fall_gradient_percent: f64,
        rise_duration: f64,
        fall_duration: f64,
        duration: f64,
    ) -> ATMGate {
        // Convert the percentage gradients into raw gradient values
        let rise_gradient: f64 = max_amplitude * rise_gradient_percent / rise_duration;
        let fall_gradient: f64 = max_amplitude * fall_gradient_percent / fall_duration;

        // Make an empty list for the pulses
        let mut pulses: Vec<Box<dyn Pulse>> = vec![];

        // Calculate the rising and falling A and B values for the tangent pulses ($A\tan(bt)$)
        let rise_b: f64 = ATMGate::find_beta(rise_duration, rise_gradient, max_amplitude);
        let rise_a: f64 = rise_gradient / rise_b;
        let fall_b: f64 = ATMGate::find_beta(-fall_duration, -fall_gradient, max_amplitude);
        let fall_a: f64 = fall_gradient / fall_b;

        // Append the tangent rise, the frequency ramp down, and the tangent fall
        pulses.push(Box::new(TangentPulse::new(
            rise_a,
            rise_b,
            0.,
            max_frequency,
            0.,
            rise_duration,
        )));
        pulses.push(Box::new(RampPulse::new(
            (max_amplitude, max_amplitude),
            (max_frequency, 0.),
            (0., 0.),
            duration - rise_duration - fall_duration,
        )));
        pulses.push(Box::new(TangentPulse::new(
            -fall_a,
            fall_b,
            fall_duration,
            0.,
            0.,
            fall_duration,
        )));
        return ATMGate {
            pulses: pulses,
            duration: duration,
        };
    }
    /// Given a duration, a gradient to start at, and a maximum amplitude to go to calculate the
    /// appropriate b value for the tangent function $a\tan(bt)$.
    fn find_beta(duration: f64, gradient: f64, max_amplitude: f64) -> f64 {
        // Set an upper and lower bound for the root finding method
        let lower_bound_b: f64 = 1e-10;
        let upper_bound_b: f64 = 15.;

        // Function whose root detones the correct b value
        let root_to_find = |b: f64| ((b * max_amplitude) / gradient).atan() - b * duration;

        // Make sure that our upper and lower bounds are above and below the x axis so there exits
        // a root
        if root_to_find(lower_bound_b) * root_to_find(upper_bound_b) > 0. {
            panic!("Bad root finding");
        }
        return root_bisection(&root_to_find, Interval::new(1e-10, 15.), None, None).unwrap();
    }
    /// Get the correct pulse index and time within that pulse given a time within the gate
    fn get_pulse_index(&self, mut time: f64) -> (usize, f64) {
        // Loop over all the pulses
        for (i, gate) in self.pulses.iter().enumerate() {
            // Get teh duration of the pulse and if its less than the pulse then return the index
            // and time
            let gate_duration: f64 = gate.get_duration();
            if time <= gate_duration {
                return (i, time);
            }
            // Otherwise subtract the duration from the time and repeat
            time -= gate_duration;
        }
        panic!("Tried to get gate index for time past the durration of the circuit.")
    }
}

impl Gate for ATMGate {
    /// Get the amplitude for the ATM Gate at a specific time
    fn get_amplitude(&self, t: f64) -> f64 {
        let (i, time) = self.get_pulse_index(t);
        return self.pulses[i].get_amplitude(time);
    }
    /// Get the frequency for the ATM Gate at a specific time
    fn get_frequency(&self, t: f64) -> f64 {
        let (i, time) = self.get_pulse_index(t);
        return self.pulses[i].get_frequency(time);
    }
    /// Get the phase for the ATM Gate at a specific time
    fn get_phase(&self, t: f64) -> f64 {
        let (i, time) = self.get_pulse_index(t);
        return self.pulses[i].get_phase(time);
    }
    /// Get the duration for the ATM Gate
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}
