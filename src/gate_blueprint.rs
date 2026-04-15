use std::collections::HashMap;

use crate::{
    gate::{CheckGateName, Gate, Idle, PiO2X, PiO2Y},
    sweep_parameter::SweepParameter,
};

use serde::de::value;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct GateBlueprint {
    name: String,
    parameters: HashMap<String, f64>,
}

impl GateBlueprint {
    pub fn from_json(
        name: String,
        json_values: &Map<String, Value>,
    ) -> (GateBlueprint, Vec<SweepParameter>) {
        let mut parameters: HashMap<String, f64> = HashMap::new();
        let mut swept_parameters: Vec<SweepParameter> = vec![];
        for (key, value) in json_values.into_iter() {
            if value.is_array() {
                swept_parameters.push(SweepParameter::new(
                    key.clone(),
                    value
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.as_f64().unwrap())
                        .collect(),
                ));
                parameters.insert(
                    key.clone(),
                    swept_parameters[swept_parameters.len() - 1].get_value(0),
                );
            } else {
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
    pub fn get_name(&self) -> &String {
        return &self.name;
    }
    pub fn get(&self, key: &str) -> f64 {
        return *self.parameters.get(key).unwrap();
    }
    pub fn get_gate(&self) -> Box<dyn Gate> {
        return try_convert_blueprint(self).unwrap();
    }
    pub fn update_parameters(&mut self, sweep_parameter: &SweepParameter, path_index: usize, value_index: usize) -> () {
        *self.parameters.get_mut(sweep_parameter.get_path(path_index)).unwrap() = sweep_parameter.get_value(value_index);
        return;
    }
}

impl From<&GateBlueprint> for Idle {
    fn from(blueprint: &GateBlueprint) -> Idle {
        return Idle::new_raw(blueprint.get("duration"));
    }
}

impl From<&GateBlueprint> for PiO2X {
    fn from(blueprint: &GateBlueprint) -> PiO2X {
        return PiO2X::new_raw();
    }
}

impl From<&GateBlueprint> for PiO2Y {
    fn from(blueprint: &GateBlueprint) -> PiO2Y {
        return PiO2Y::new_raw();
    }
}

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

fn try_convert_blueprint(mut blueprint: &GateBlueprint) -> Option<Box<dyn Gate>> {
    static DICT_LOADERS: &[fn(&GateBlueprint) -> Result<Box<dyn Gate>, &GateBlueprint>] =
        &[
        try_convert_blueprint_to::<Idle>,
        try_convert_blueprint_to::<PiO2X>,
        try_convert_blueprint_to::<PiO2Y>,
        ];

    for loader in DICT_LOADERS {
        match loader(blueprint) {
            Ok(c) => return Some(c),
            Err(c) => blueprint = c,
        };
    }
    None
}
