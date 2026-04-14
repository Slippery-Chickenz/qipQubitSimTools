use std::{env, io::BufReader, fs::File};

extern crate blas_src;
extern crate serde_json;


// use qip_qst::experiment::Experiment;
use qip_qst::{
    circuit::Circuit,
    experiment::{RamseyEndGate, RamseyExperiment},
    gate::{Idle, PiO2X},
    qubit_array::QubitArray,
    simulation_results::SimulationResults,
    simulator::Simulator,
    circuit_blueprint::CircuitBlueprint,
};

use ndarray::Array1;

use serde_json::Value;

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    // let ramsey_exp: RamseyExperiment = RamseyExperiment::new(
    //     RamseyEndGate::X,
    //     Array1::<f64>::linspace(-1., 1., 100).to_vec(),
    //     Array1::<f64>::linspace(0., 2., 100).to_vec(),
    // );
    // ramsey_exp.run_experiment().unwrap();

    json_example();
}

fn json_example() -> () {

    let file = File::open("circuit.json").unwrap();
    let reader = BufReader::new(file);

    let u: Value = serde_json::from_reader(reader).unwrap();

    let _test_blueprint = CircuitBlueprint::from_json(u);
    return;
}

fn simulator_example() -> () {
    // Simulator
    let mut simulator: Simulator = Simulator::new();

    // Circuit
    let mut circuit: Circuit = Circuit::new();
    circuit.add_gate(PiO2X::new());
    circuit.add_gate(Idle::new(0.1));
    circuit.add_gate(PiO2X::new());

    // QubitArray
    let qubit_array: QubitArray = QubitArray::new(1, 0., 0.);

    let results: SimulationResults = simulator.simulate_circuit(circuit, qubit_array, 1000, 2);

    results.save_bloch_coords_cart("Test_Sim.txt").unwrap();
}
