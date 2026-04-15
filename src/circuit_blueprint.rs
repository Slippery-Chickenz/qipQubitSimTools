use std::collections::HashMap;

use crate::{
    circuit::Circuit, gate, gate_blueprint::GateBlueprint, sweep_parameter::SweepParameter,
};

use serde_json::{Map, Value};

#[derive(Debug)]
pub struct CircuitBlueprint {
    circuit_data: Vec<GateBlueprint>,
    gate_directory: HashMap<String, usize>,
}

impl CircuitBlueprint {
    pub fn from_json(json_values: &Map<String, Value>) -> (CircuitBlueprint, Vec<SweepParameter>) {
        // Get list of gates defining the circuit
        let order: &Vec<Value> = json_values["order"].as_array().unwrap();

        // Get object map to map gates in order to how they are defined
        let gate_map: &Map<String, Value> = json_values["gates"].as_object().unwrap();

        // Cicuit data for the returned blueprint
        let mut circuit_data: Vec<GateBlueprint> = vec![];
        let mut gate_directory: HashMap<String, usize> = HashMap::new();

        let mut gate_swept_parameters: Vec<SweepParameter> = vec![];

        for (i, gate) in order.iter().enumerate() {
            let (gate_blueprint, mut swept_parameters): (GateBlueprint, Vec<SweepParameter>) =
                GateBlueprint::from_json(
                    gate.as_str().unwrap().to_string(),
                    &gate_map[gate.as_str().unwrap()].as_object().unwrap(),
                );
            let gate_name: String = gate_blueprint.get_name().clone() + "_" + &i.to_string();
            gate_directory.insert(gate_name.clone(), i);
            circuit_data.push(gate_blueprint);
            for sweep_parameter in &mut swept_parameters {
                sweep_parameter.add_path(gate_name.clone());
            }
            gate_swept_parameters.append(&mut swept_parameters);
        }

        return (
            CircuitBlueprint {
                circuit_data: circuit_data,
                gate_directory: gate_directory,
            },
            gate_swept_parameters,
        );
    }
    pub fn get_circuit(&self) -> Circuit {
        // Circuit object to construct
        let mut circuit: Circuit = Circuit::new();

        for gate_blueprint in &self.circuit_data {
            circuit.add_gate(gate_blueprint.get_gate());
        }
        return circuit;
    }
    pub fn update_parameters(&mut self, sweep_parameter: &SweepParameter, path_index: usize, value_index: usize) -> () {
        self.circuit_data[self.gate_directory[sweep_parameter.get_path(path_index)]].update_parameters(sweep_parameter, 2, value_index);
        return;
    }
}
