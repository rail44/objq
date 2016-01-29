use std::io::Write;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use serde_json;
use serde_json::value::Value as JsonValue;
use yaml_rust::{Yaml as YamlValue, YamlEmitter};

#[derive(Debug)]
pub struct Yaml;

impl Yaml {
    pub fn new(_: Vec<&str>) -> Yaml {
        Yaml 
    }
}

impl super::Output for Yaml {
    fn output<T: Write>(self, w: &mut T, value: &JsonValue) -> Result<(), serde_json::Error> {
        let mut out = String::new();
        YamlEmitter::new(&mut out).dump(&to_yaml_value(value)).unwrap_or_else(|_| unreachable!());
        write!(w, "{}", out).map_err(|e| serde_json::Error::IoError(e))
    }
}

fn to_yaml_value(value: &JsonValue) -> YamlValue {
    match value {
        &JsonValue::Null => YamlValue::Null,
        &JsonValue::Bool(ref b) => YamlValue::Boolean(b.clone()),
        &JsonValue::I64(ref i) => YamlValue::Integer(i.clone()),
        &JsonValue::U64(ref u) => YamlValue::Integer(u.clone() as i64),
        &JsonValue::F64(ref f) => YamlValue::Real(format!("{}", f)),
        &JsonValue::String(ref s) => YamlValue::String(s.clone()),
        &JsonValue::Array(ref arr) => {
            let yaml_arr = arr.iter().map(|v| to_yaml_value(&v)).collect();
            YamlValue::Array(yaml_arr)
        },
        &JsonValue::Object(ref b_tree_map) => {
            let iter = b_tree_map.iter().map(|(k, v)| (YamlValue::String(k.clone()), to_yaml_value(v)));
            YamlValue::Hash(BTreeMap::from_iter(iter))
        }
    }
}
