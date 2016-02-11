use std::io;
use std::io::Write;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use yaml_rust::{Yaml as YamlValue, YamlEmitter};
use super::Value;

#[derive(Debug)]
pub struct Yaml;

impl Yaml {
    pub fn new(_: Vec<&str>) -> Yaml {
        Yaml 
    }
}

pub type Error = io::Error;

impl super::Output for Yaml {
    type Error = Error;

    fn output<T: Write>(self, w: &mut T, value: &Value) -> Result<(), Self::Error> {
        let mut out = String::new();
        YamlEmitter::new(&mut out).dump(&to_yaml_value(value)).unwrap_or_else(|_| unreachable!());
        write!(w, "{}", out)
    }
}

fn to_yaml_value(value: &Value) -> YamlValue {
    match value {
        &Value::Null => YamlValue::Null,
        &Value::Bool(ref b) => YamlValue::Boolean(b.clone()),
        &Value::I64(ref i) => YamlValue::Integer(i.clone()),
        &Value::U64(ref u) => YamlValue::Integer(u.clone() as i64),
        &Value::F64(ref f) => YamlValue::Real(format!("{}", f)),
        &Value::String(ref s) => YamlValue::String(s.clone()),
        &Value::Array(ref arr) => {
            let yaml_arr = arr.iter().map(|v| to_yaml_value(&v)).collect();
            YamlValue::Array(yaml_arr)
        },
        &Value::Object(ref b_tree_map) => {
            let iter = b_tree_map.iter().map(|(k, v)| (YamlValue::String(k.clone()), to_yaml_value(v)));
            YamlValue::Hash(BTreeMap::from_iter(iter))
        }
    }
}
