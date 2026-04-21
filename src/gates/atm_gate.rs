use crate::default_name;
use crate::gates::{CheckGateName, Gate};
use crate::pulses::{Pulse, Ramp, Tangent};

use rootfinder::{Interval, root_bisection};

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
        pulses.push(Box::new(Tangent::new(
            rise_a,
            rise_b,
            0.,
            max_frequency,
            0.,
            rise_duration,
        )));
        pulses.push(Box::new(Ramp::new(
            (max_amplitude, max_amplitude),
            (max_frequency, 0.),
            (0., 0.),
            duration - rise_duration - fall_duration,
        )));
        pulses.push(Box::new(Tangent::new(
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
