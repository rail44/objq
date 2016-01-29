use std::io::Read;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use yaml_rust::{Yaml, YamlLoader};
use serde_json;
use serde_json::Value;

pub fn to_json_value(yaml: Yaml) -> Value {
    match yaml {
        Yaml::Null => Value::Null,
        Yaml::Boolean(b) => Value::Bool(b),
        Yaml::Integer(i) => Value::I64(i),
        Yaml::Real(f_str) => Value::F64(f_str.parse().unwrap()),
        Yaml::String(s) => Value::String(s),
        Yaml::Array(arr) => {
            let json_value_arr = arr.into_iter().map(|v| to_json_value(v)).collect();
            Value::Array(json_value_arr)
        },
        Yaml::Hash(b_tree_map) => {
            let iter = b_tree_map.into_iter().map(|(k, v)| (k.as_str().unwrap().to_string(), to_json_value(v)));
            Value::Object(BTreeMap::from_iter(iter))
        },
        Yaml::Alias(_) => unimplemented!(),
        Yaml::BadValue => panic!(),
    }
}

pub fn from_reader<T: Read>(r: &mut T) -> Result<Value, serde_json::Error> {
    let mut in_str = String::new();
    try!(r.read_to_string(&mut in_str).map_err(|e| serde_json::Error::IoError(e)));
    let yaml = YamlLoader::load_from_str(in_str.as_str())
        .unwrap_or_else(|_| unimplemented!())
        .pop()
        .unwrap();
    Ok(to_json_value(yaml))
}
