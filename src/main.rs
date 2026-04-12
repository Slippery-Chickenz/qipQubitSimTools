extern crate blas_src;
extern crate serde_json;

use std::env;

use qip_qst::experiment::Experiment;
use qip_qst::gate::PiO2X;
use qip_qst::larmor_sweep_results::LarmorSweepResult;
use qip_qst::qubit_array::QubitArray;
use qip_qst::simulation_results::SimulationResults;
use qip_qst::simulator::Simulator;
use qip_qst::{circuit::Circuit, gate::IdleGate};

use ndarray::{Array1, array};

use num_complex::Complex64;

use serde_json::Value;

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    // Experiment
    let experim: Experiment = Experiment {};

    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    println!("{}", v);

    // Simulator
    let mut simulator: Simulator = Simulator::new();

    // Circuit
    let mut circuit: Circuit = Circuit::new();
    circuit.add_gate(Box::new(PiO2X::new()));
    circuit.add_gate(Box::new(IdleGate::new(0.1)));
    circuit.add_gate(Box::new(PiO2X::new()));

    // QubitArray
    let qubit_array: QubitArray = QubitArray::new(1, 0.);

    let results: SimulationResults = simulator.simulate_circuit(circuit, qubit_array, 0., 1000, 2);

    for density_matrix in results.get_density_matrices() {
        println!("{:.2}", density_matrix);
        println!();
    }
    results.save_bloch_coords_cart("Test_Sim.txt").unwrap();

    println!("{}", results.get_final_probability());
}
