
mod gate;

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

    // Gates for the circuit
    let pio2x_gate: Box<gate::PiO2X> = Box::new( PiO2X {  } );
    let idle_gate: Box<gate::IdleGate> = Box::new( IdleGate { length: 1. } );
    let pio2y_gate: Box<gate::PiO2Y> = Box::new( PiO2Y {  } );

    // Circuit to run
    let circuit: Vec<Box<dyn Gate>> = vec![ pio2x_gate, idle_gate, pio2y_gate ];

    // List of integrated frequencies over the length of the circuit
    let integrated_frequencies: Vec<f64> = Vec::with_capacity(NUM_ITERATIONS);

    // Integrate through all the frequencies
    for t in times {
        let temp_f = 
        for f in integrated_frequencies {

        }
    }

    println!("{}", inital_state);

    for _t in times {
        println!();
    }

}













