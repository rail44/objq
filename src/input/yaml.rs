use std::io;
use std::io::Read;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use yaml_rust::{Yaml as YamlValue, YamlLoader};
use super::Value;

#[derive(Debug)]
pub struct Yaml;

impl Yaml {
    pub fn new() -> Yaml {
        Yaml
    }
}

pub fn to_json_value(yaml: YamlValue) -> Value {
    match yaml {
        YamlValue::Null => Value::Null,
        YamlValue::Boolean(b) => Value::Bool(b),
        YamlValue::Integer(i) => Value::I64(i),
        YamlValue::Real(f_str) => Value::F64(f_str.parse().unwrap()),
        YamlValue::String(s) => Value::String(s),
        YamlValue::Array(arr) => {
            let json_value_arr = arr.into_iter().map(|v| to_json_value(v)).collect();
            Value::Array(json_value_arr)
        },
        YamlValue::Hash(b_tree_map) => {
            let iter = b_tree_map.into_iter().map(|(k, v)| (to_string(k), to_json_value(v)));
            Value::Object(BTreeMap::from_iter(iter))
        },
        YamlValue::Alias(_) => unimplemented!(),
        YamlValue::BadValue => panic!(),
    }
}

fn to_string(yaml: YamlValue) -> String {
    match yaml {
        YamlValue::Boolean(b) => b.to_string(),
        YamlValue::Integer(i) => i.to_string(),
        YamlValue::Real(f_str) => f_str,
        YamlValue::String(s) => s,
        _ => unimplemented!(),
    }
}

pub type Error = io::Error;

impl super::Input for Yaml {
    type Error = Error;

    fn input<T: Read>(self, r: &mut T) -> Result<Value, Self::Error> {
        let mut in_str = String::new();
        try!(r.read_to_string(&mut in_str));
        let yaml = YamlLoader::load_from_str(in_str.as_str())
            .unwrap_or_else(|_| unimplemented!())
            .pop()
            .unwrap();
        Ok(to_json_value(yaml))
    }
}
