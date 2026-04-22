use std::collections::HashMap;

use crate::blueprints::GateBlueprint;
use crate::experiment::SweepParameter;
use crate::simulation::Circuit;

use serde_json::{Map, Value};

/// Object to hold all the data needed to construct a quantum circuit alongside functions to update
/// the parameters of the circuit
#[derive(Debug)]
pub struct CircuitBlueprint {
    /// Circuit data which is a vector of blueprints for each gate in the circuit
    circuit_data: Vec<GateBlueprint>,
    /// Gate directory to map human readable gate names to their position in the vector
    gate_directory: HashMap<String, usize>,
}

impl CircuitBlueprint {
    /// Construct a CircuitBlueprint from a map of Strings to json values. This will return not
    /// only the circuit blueprint but also a vector of parameters set in the json values to be
    /// swept over.
    pub fn from_json(json_values: &Map<String, Value>) -> (CircuitBlueprint, Vec<SweepParameter>) {
        // Get list of gates defining the circuit. Expected the "order" key to be an array of vectors
        let order: &Vec<Value> = json_values["order"].as_array().unwrap();

        // Get object map to map gates in order to how they are defined
        let gate_map: &Map<String, Value> = json_values["gates"].as_object().unwrap();

        // Cicuit data for the returned blueprint
        let mut circuit_data: Vec<GateBlueprint> = vec![];
        let mut gate_directory: HashMap<String, usize> = HashMap::new();

        // Vector of parameters to be swept over
        let mut gate_swept_parameters: Vec<SweepParameter> = vec![];

        // Loop over the array of gates
        for (i, gate) in order.iter().enumerate() {
            // String representing the gate
            let gate_name: &str = gate.as_str().unwrap();
            // Get a blueprint and the swept parameters for the given gate defined by the string
            let (gate_blueprint, mut swept_parameters): (GateBlueprint, Vec<SweepParameter>) =
                GateBlueprint::from_json(
                    gate_name.to_string(),
                    &gate_map[gate_name].as_object().unwrap(),
                );

            // Path for the added gate. This is to keep track of where to go to update the
            // parameters later. This includes a number to track if there is more than one of the
            // same gate.
            let gate_path: String = gate_blueprint.get_name().clone() + "_" + &i.to_string();

            // Insert the gate path to the directory to map it to the correct index in the vector
            gate_directory.insert(gate_path.clone(), i);

            // Add the blueprint to the vector of data
            circuit_data.push(gate_blueprint);

            // Add to the path in the sweep parameter to track it for updates later
            for sweep_parameter in &mut swept_parameters {
                sweep_parameter.add_path(gate_path.clone());
            }
            // Append the sweep parameters from this gate to the overall
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
    pub fn update_parameters(
        &mut self,
        sweep_parameter: &SweepParameter,
        path_index: usize,
        value_index: usize,
    ) -> () {
        self.circuit_data[self.gate_directory[sweep_parameter.get_path(path_index)]]
            .update_parameters(sweep_parameter, 2, value_index);
        return;
    }
}
