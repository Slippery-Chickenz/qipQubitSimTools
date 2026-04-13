use crate::gate_blueprint::GateBlueprint;

use serde_json::{ Value, Map };

#[derive(Debug)]
pub struct CircuitBlueprint {
    circuit_data: Vec<GateBlueprint>,
}

impl CircuitBlueprint {
    pub fn from_json(json_values: Value) -> CircuitBlueprint {

        // Get list of gates defining the circuit
        let order: &Vec<Value> = json_values["order"].as_array().unwrap();

        // Get object map to map gates in order to how they are defined
        let gate_map: &Map<String, Value> = json_values["gates"].as_object().unwrap();

        // Cicuit data for the returned blueprint
        let mut circuit_data: Vec<GateBlueprint> = vec![];

        for gate in order {
            circuit_data.push(GateBlueprint::from_json(gate.as_str().unwrap().to_string(), &gate_map[gate.as_str().unwrap()].as_object().unwrap()));
        }

        dbg!(&circuit_data);
        return CircuitBlueprint { circuit_data: circuit_data }
    }
}
