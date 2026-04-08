use std::env;

use qip_qst::circuit::Circuit;
use qip_qst::simulator::Simulator;
use qip_qst::simulation_results::SimulationResults;
use qip_qst::qubit_array::QubitArray;
use qip_qst::gate::PiO2X;

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

}
