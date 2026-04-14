use std::collections::HashMap;

use crate::gate::{CheckGateName, Gate, Idle};

use serde_json::{Map, Value};

#[derive(Debug)]
pub struct GateBlueprint {
    name: String,
    parameters: HashMap<String, f64>,
}

impl GateBlueprint {
    pub fn from_json(name: String, json_values: &Map<String, Value>) -> GateBlueprint {
        let mut parameters: HashMap<String, f64> = HashMap::new();
        for (key, value) in json_values.into_iter() {
            parameters.insert(key.clone(), value.as_f64().unwrap());
        }
        return GateBlueprint {
            name: name,
            parameters: parameters,
        };
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
}

impl From<&GateBlueprint> for Idle {
    fn from(blueprint: &GateBlueprint) -> Idle {
        return Idle::new_raw(blueprint.get("duration"));
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
        &[try_convert_blueprint_to::<Idle>];

    for loader in DICT_LOADERS {
        match loader(blueprint) {
            Ok(c) => return Some(c),
            Err(c) => blueprint = c,
        };
    }
    None
}
