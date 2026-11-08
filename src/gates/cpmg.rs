use crate::default_name;
use crate::gates::{CheckGateName, Gate, Idle, PiX};

/// Struct for a gate which just idles
pub struct CPMG {
    /// Made up of a dynamic number of Idle and PiX Gates
    gates: Vec<Box<dyn Gate>>,
    /// Duration of the gate
    duration: f64,
}

impl CPMG {
    /// Get a Box to an Idle gate given a duration
    pub fn new(tau: f64, num_pi: usize) -> Box<CPMG> {
        return Box::new(CPMG::new_raw(tau, num_pi));
    }
    /// Get a raw Idle gate object given a duration
    pub fn new_raw(tau: f64, num_pi: usize) -> CPMG {
        // Special case of no pi gates
        if num_pi == 0 {
            return CPMG {
                gates: vec![Idle::new(tau)],
                duration: tau,
            };
        }

        let idle_time: f64 = tau / (num_pi * 2) as f64;

        let mut gates: Vec<Box<dyn Gate>> = vec![Idle::new(idle_time), PiX::new()];
        let pi_x_duration = gates[1].get_duration();

        for _i in 0..num_pi - 1 {
            gates.push(Idle::new(idle_time * 2.));
            gates.push(PiX::new());
        }

        gates.push(Idle::new(idle_time));

        return CPMG {
            gates: gates,
            duration: tau + (pi_x_duration * num_pi as f64),
        };
    }
    /// Get the correct gate index and time within that gate given a time within the gate
    fn get_gate_index(&self, mut time: f64) -> (usize, f64) {
        // Loop over all the pulses
        for (i, gate) in self.gates.iter().enumerate() {
            // Get the duration of the pulse and if its less than the gate then return the index
            // and time
            let gate_duration: f64 = gate.get_duration();
            // if time <= gate_duration {
            if (time - gate_duration) <= 1e-10 {
                return (i, time);
            }
            // Otherwise subtract the duration from the time and repeat
            time -= gate_duration;
        }
        panic!("Tried to get gate index for time past the durration of the circuit.")
    }
}

// Default name for the idle gate
default_name!(CPMG);

impl Gate for CPMG {
    /// Get the amplitude for the ATM Gate at a specific time
    fn get_amplitude(&self, t: f64) -> f64 {
        let (i, time) = self.get_gate_index(t);
        return self.gates[i].get_amplitude(time);
    }
    /// Get the frequency for the ATM Gate at a specific time
    fn get_frequency(&self, t: f64) -> f64 {
        let (i, time) = self.get_gate_index(t);
        return self.gates[i].get_frequency(time);
    }
    /// Get the phase for the ATM Gate at a specific time
    fn get_phase(&self, t: f64) -> f64 {
        let (i, time) = self.get_gate_index(t);
        return self.gates[i].get_phase(time);
    }
    /// Get the duration for the ATM Gate
    fn get_duration(&self) -> f64 {
        return self.duration;
    }
}
