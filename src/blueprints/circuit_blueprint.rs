use std::collections::HashMap;
use std::{fs, io::BufReader};

use crate::blueprints::GateBlueprint;
use crate::experiment::SweepParameter;
use crate::simulation::Circuit;

use serde_json::{Map, Value};

/// Object to hold all the data needed to construct a quantum circuit alongside functions to update
/// the parameters of the circuit
#[derive(Debug)]
pub struct CircuitBlueprint {
    /// Circuit data which is a vector of blueprints for each gate in the circuit
    circuit_data: Vec<String>,
    /// Gate directory to map human readable gate names to their position in the vector
    gate_directory: HashMap<String, GateBlueprint>,
}

impl CircuitBlueprint {
    /// Construct a CircuitBlueprint from a map of Strings to json values. This will return not
    /// only the circuit blueprint but also a vector of parameters set in the json values to be
    /// swept over.
    pub fn from_json(
        mut json_values: Map<String, Value>,
    ) -> (CircuitBlueprint, Vec<SweepParameter>) {
        // If there is a key in the map called "filename" then assume that is a seperate
        // file defining the circuit and look there
        if json_values.contains_key("filename") {
            let circuit_file: fs::File =
                fs::File::open(json_values["filename"].as_str().unwrap()).unwrap();
            let circuit_reader: BufReader<fs::File> = BufReader::new(circuit_file);
            json_values = serde_json::from_reader(circuit_reader).unwrap();
        }

        // Ordered vector of gate names to construct the circuit
        let circuit_data: Vec<String> =
            serde_json::from_value(json_values.remove("order").unwrap()).unwrap();

        // Map the circuit name strings to a blueprint to construct the circuit objects
        let mut gate_directory: HashMap<String, GateBlueprint> = HashMap::new();

        // Vector of parameters to be swept over
        let mut gate_swept_parameters: Vec<SweepParameter> = vec![];

        // Loop over the array of gates
        // for gate_values in json_values["gates"].as_array().unwrap().iter() {
        for gate_values in
            serde_json::from_value::<Vec<Map<String, Value>>>(json_values.remove("gates").unwrap())
                .unwrap()
                .iter_mut()
        {
            // String representing the name of the gate
            let gate_name: String =
                serde_json::from_value(gate_values.remove("name").unwrap()).unwrap();

            // String representing the type of gate to add if there is no type specified then assume
            // it is the same as the name
            let gate_type: String = if gate_values.contains_key("type") {
                serde_json::from_value(gate_values.remove("type").unwrap()).unwrap()
            } else {
                gate_name.clone()
            };

            let gate_data: Map<String, Value> =
                serde_json::from_value(gate_values.remove("parameters").unwrap_or_default())
                    .unwrap_or_default();

            // Get a blueprint and the swept parameters for the given gate defined by the string
            let (gate_blueprint, mut swept_parameters): (GateBlueprint, Vec<SweepParameter>) =
                GateBlueprint::from_json(gate_type.to_string(), gate_data);

            gate_directory.insert(gate_name.to_string(), gate_blueprint);

            // Add to the path in the sweep parameter to track it for updates later
            for sweep_parameter in &mut swept_parameters {
                sweep_parameter.add_path(gate_name.to_string());
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

        for gate_name in &self.circuit_data {
            circuit.add_gate(self.gate_directory.get(gate_name).unwrap().get_gate());
        }
        return circuit;
    }
    pub fn update_parameters(
        &mut self,
        sweep_parameter: &SweepParameter,
        path_index: usize,
        value_index: usize,
    ) -> () {
        self.gate_directory
            .get_mut(sweep_parameter.get_path(path_index))
            .unwrap()
            .update_parameters(sweep_parameter, 2, value_index);
        return;
    }
}
