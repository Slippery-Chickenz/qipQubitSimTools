use std::env;

use qip_qst::circuit::Circuit;
use qip_qst::simulator::Simulator;
use qip_qst::simulation_results::SimulationResults;
use qip_qst::qubit_array::QubitArray;
use qip_qst::gate::PiO2X;

use ndarray::{ array, Array1, Array2 };
use ndarray_linalg::trace::Trace;

use num_complex::Complex64 as c64;

extern crate blas_src;

fn main() {

    unsafe{
        env::set_var("RUST_BACKTRACE", "1");
    }

    // Simulator
    let mut simulator: Simulator = Simulator::new();

    // Circuit
    let mut circuit: Circuit = Circuit::new();
    circuit.add_gate(Box::new(PiO2X::new()));
    circuit.add_gate(Box::new(PiO2X::new()));

    // QubitArray
    let mut qubit_array: QubitArray = QubitArray::new(1, 0.);

    let results: SimulationResults = simulator.simulate_circuit(&mut circuit, &mut qubit_array, 0., 1000, 5);

    for density_matrix in results.get_array().get_density_matrices() {
        println!("{:.2}", density_matrix);
        println!();
    }



    let test1: Array1<c64> = array![c64::new(1./ 2.0f64.sqrt(), 0.), c64::new(0., 1./2.0f64.sqrt())];

    let test_proj: Array2<c64> = test1.to_shape([2, 1]).unwrap().dot(&(test1.mapv(|x| x.conj()).to_shape([1, 2]).unwrap()));

    let test_dens: Array2<c64> = array![[c64::new(0.5, 0.), c64::new(0.5, 0.)], [c64::new(0.5, 0.), c64::new(0.5, 0.)]];

    let expect: Array2<c64> = test_dens.dot(&test_proj);

    println!("{}", test_proj);
    println!("{}", test_dens.dot(&test_proj));
    println!("{}", expect.trace().unwrap());
}
