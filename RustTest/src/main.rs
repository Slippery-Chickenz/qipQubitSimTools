// use qip_qst::gate;
// use qip_qst::circuit::Circuit;

use ndarray::{ Array2, Array3 };
use num_complex::Complex64;

fn main() {

    // Larmor value of the qubit
    let _larmor: f64 = 0.0;

    // Inital state for the qubit
    let mut test1: Array2::<f64> = Array2::<f64>::zeros((2,2));
    test1[[0, 0]] = 1.0;

    // Inital state for the qubit
    let mut test2: Array2::<f64> = Array2::<f64>::zeros((2,2));
    test2[[1, 1]] = 1.0;

    let test3: Array3<Complex64> = Array3::<Complex64>::from_elem((3, 2, 2), Complex64 { re: 2., im: 0. } );

    // // Time values for the simulation
    // const NUM_ITERATIONS: usize = 1000;
    // const SIM_LENGTH: f64 = 10.;
    // println!("{}, {}", NUM_ITERATIONS, SIM_LENGTH);
    //
    // // Gates for the circuit
    // let pio2x_gate: Box<gate::PiO2X> = Box::new( gate::PiO2X {  } );
    // let idle_gate: Box<gate::IdleGate> = Box::new( gate::IdleGate::new(1.) );
    // let pio2y_gate: Box<gate::PiO2Y> = Box::new( gate::PiO2Y {  } );
    //
    // // Circuit to run
    // let mut circuit: Circuit = Circuit::new();
    // circuit.add_gate(pio2x_gate);
    // circuit.add_gate(idle_gate);
    // circuit.add_gate(pio2y_gate);

    println!("{}", test1);
    println!("{}", test2);
    println!("{}", test1 + test2);

    let mut temp_a: Array2<Complex64> = Array2::<Complex64>::zeros((2, 2));
    temp_a[[0, 0]] = Complex64 { re: 1., im: 0. };
    temp_a[[1, 1]] = Complex64 { re: 1., im: 0. };

    for a in test3.outer_iter() {
        println!("{} @ {}", temp_a, a);
        temp_a = temp_a.dot(&a);
    }
    println!("{}", temp_a);
}
