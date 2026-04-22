use std::collections::HashMap;

use crate::gates::{ATMGate, CheckGateName, Constant, Gate, Idle, PiO2X, PiO2Y, PiX, PiY};

use crate::experiment::SweepParameter;

use serde_json::{Map, Value};

/// Struct to hold the parameters and values needed to construct a gate
#[derive(Debug)]
pub struct GateBlueprint {
    /// Name of the gate as a string
    name: String,
    /// Parameters and values to use for those parameters when constructing the gate
    parameters: HashMap<String, f64>,
}

impl GateBlueprint {
    /// Get a GateBlueprint object given a name and a map of json values that define the parameters
    /// and values for those parameters. Returns the GateBlueprint alongside a vector of parameters
    /// that are defined to be swept over
    pub fn from_json(
        name: String,
        json_values: &Map<String, Value>,
    ) -> (GateBlueprint, Vec<SweepParameter>) {
        // Parameters to construct the gate with
        let mut parameters: HashMap<String, f64> = HashMap::new();
        // Parameters to be swept over
        let mut swept_parameters: Vec<SweepParameter> = vec![];

        // Loop through all the keys and valeus in the json map given
        for (key, value) in json_values.into_iter() {
            // If value is not a number then it must be something to be swept over
            if !value.is_number() {
                // Add a new sweep parameter defined with the key name and the values in the json
                swept_parameters.push(SweepParameter::from_json(key.clone(), value));
                // Add the parameter to the blueprint with the value set to the first defined in
                // the sweep
                parameters.insert(
                    key.clone(),
                    swept_parameters[swept_parameters.len() - 1].get_value(0),
                );
            } else {
                // If it is a number then just insert it into the blueprint
                parameters.insert(key.clone(), value.as_f64().unwrap());
            }
        }
        return (
            GateBlueprint {
                name: name,
                parameters: parameters,
            },
            swept_parameters,
        );
    }
    /// Get the name  of the gate this is a blueprint for
    pub fn get_name(&self) -> &String {
        return &self.name;
    }
    /// Get the value of a specific parameter key
    pub fn get(&self, key: &str) -> f64 {
        return *self.parameters.get(key).unwrap();
    }
    /// Get a Box to a gate object defined by this blueprint
    pub fn get_gate(&self) -> Box<dyn Gate> {
        return try_convert_blueprint(self).unwrap();
    }
    /// Update the parameters in this blueprint
    pub fn update_parameters(
        &mut self,
        sweep_parameter: &SweepParameter,
        path_index: usize,
        value_index: usize,
    ) -> () {
        *self
            .parameters
            .get_mut(sweep_parameter.get_path(path_index))
            .unwrap() = sweep_parameter.get_value(value_index);
        return;
    }
}

impl From<&GateBlueprint> for Idle {
    /// Convert from a reference to a gate blueprint to an Idle Gate
    fn from(blueprint: &GateBlueprint) -> Idle {
        return Idle::new_raw(blueprint.get("duration"));
    }
}

impl From<&GateBlueprint> for Constant {
    /// Convert from a reference to a gate blueprint to a Constant gate
    fn from(blueprint: &GateBlueprint) -> Constant {
        return Constant::new_raw(
            blueprint.get("amplitude"),
            blueprint.get("frequency"),
            blueprint.get("phase"),
            blueprint.get("duration"),
        );
    }
}

impl From<&GateBlueprint> for PiO2X {
    /// Convert from a reference to a gate blueprint to a PiO2X Gate
    fn from(_blueprint: &GateBlueprint) -> PiO2X {
        return PiO2X::new_raw();
    }
}

impl From<&GateBlueprint> for PiX {
    /// Convert from a reference to a gate blueprint to a PiO2X Gate
    fn from(_blueprint: &GateBlueprint) -> PiX {
        return PiX::new_raw();
    }
}

impl From<&GateBlueprint> for PiO2Y {
    /// Convert from a reference to a gate blueprint to a PiO2Y Gate
    fn from(_blueprint: &GateBlueprint) -> PiO2Y {
        return PiO2Y::new_raw();
    }
}

impl From<&GateBlueprint> for PiY {
    /// Convert from a reference to a gate blueprint to a PiO2X Gate
    fn from(_blueprint: &GateBlueprint) -> PiY {
        return PiY::new_raw();
    }
}

impl From<&GateBlueprint> for ATMGate {
    /// Convert from a reference to a gate blueprint to an ATM Gate
    fn from(blueprint: &GateBlueprint) -> ATMGate {
        return ATMGate::new_raw(
            blueprint.get("max_amplitude"),
            blueprint.get("max_frequency"),
            blueprint.get("rise_gradient"),
            blueprint.get("fall_gradient"),
            blueprint.get("rise_time"),
            blueprint.get("fall_time"),
            blueprint.get("duration"),
        );
    }
}

/// Function to try and convert from a blueprint to a Box to a gate object
/// If it finds a conversion from the blueprint to the gate it returns a Box to the gate if not it
/// errors and returns back the reference to the blueprint object
fn try_convert_blueprint_to<T>(blueprint: &GateBlueprint) -> Result<Box<dyn Gate>, &GateBlueprint>
where
    T: Gate + CheckGateName + 'static,
    for<'a> &'a GateBlueprint: Into<T>,
{
    if T::check_name(blueprint.get_name()) {
        Ok(Box::new(blueprint.into()))
    } else {
        Err(blueprint)
    }
}

/// Function to try and convert a blueprint object. If it finds a correct conversion it will return
/// the Box to that Gate otherwise it will return a None object
fn try_convert_blueprint(mut blueprint: &GateBlueprint) -> Option<Box<dyn Gate>> {
    // List of functiosn to try and use to convert to a Gate
    static DICT_LOADERS: &[fn(&GateBlueprint) -> Result<Box<dyn Gate>, &GateBlueprint>] = &[
        try_convert_blueprint_to::<Idle>,
        try_convert_blueprint_to::<Constant>,
        try_convert_blueprint_to::<PiO2X>,
        try_convert_blueprint_to::<PiX>,
        try_convert_blueprint_to::<PiO2Y>,
        try_convert_blueprint_to::<PiY>,
        try_convert_blueprint_to::<ATMGate>,
    ];
    // Loop over the functiosn and try each one to convert
    for loader in DICT_LOADERS {
        match loader(blueprint) {
            Ok(c) => return Some(c),
            Err(c) => blueprint = c,
        };
    }
    None
}
