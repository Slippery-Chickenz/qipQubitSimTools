
use ndarray::{Array1, Array2};

fn main() {

    // Larmor value of the qubit
    let larmor: f64 = 0.0;

    // Inital state for the qubit
    let mut inital_state: Array2::<f64> = Array2::<f64>::zeros((2,2));
    inital_state[[0, 0]] = 1.0;

    // Time values for the simulation
    const NUM_ITERATIONS: usize = 1000;
    const SIM_LENGTH: f64 = 10.;
    let times: Array1::<f64> = Array1::<f64>::linspace(0., SIM_LENGTH, NUM_ITERATIONS);

    // ConstantPulse
    let test_constant: ConstantPulse = ConstantPulse { frequency: 1., amplitude: 1., length: 1. };
    
    println!("{}", inital_state);

    for t in times {
        println!("{}, {}, {}, {}", test_constant.get_frequency(t), test_constant.get_amplitude(t), test_constant.get_length(), larmor);
    }

}

trait Pulse {
    fn get_frequency(&self, t: f64) -> f64;
    fn get_amplitude(&self, t: f64) -> f64;
    fn get_length(&self) -> f64;
}

trait Gate {
    fn get_frequency(&self, t: f64) -> f64;
    fn get_amplitude(&self, t: f64) -> f64;
}

struct ConstantPulse {
    frequency: f64,
    amplitude: f64,
    length: f64,
}

impl Pulse for ConstantPulse {
    fn get_frequency(&self, _t: f64) -> f64 {
        return self.frequency
    }
    fn get_amplitude(&self, _t: f64) -> f64 {
        return self.amplitude
    }
    fn get_length(&self) -> f64 {
        return self.length
    }
}
