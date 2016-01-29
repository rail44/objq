use std::io::Write;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use serde_json;
use serde_json::value::Value;
use yaml_rust::{Yaml, YamlEmitter};

fn to_yaml(value: &Value) -> Yaml {
    match value {
        &Value::Null => Yaml::Null,
        &Value::Bool(ref b) => Yaml::Boolean(b.clone()),
        &Value::I64(ref i) => Yaml::Integer(i.clone()),
        &Value::U64(ref u) => Yaml::Integer(u.clone() as i64),
        &Value::F64(ref f) => Yaml::Real(format!("{}", f)),
        &Value::String(ref s) => Yaml::String(s.clone()),
        &Value::Array(ref arr) => {
            let yaml_arr = arr.iter().map(|v| to_yaml(&v)).collect();
            Yaml::Array(yaml_arr)
        },
        &Value::Object(ref b_tree_map) => {
            let iter = b_tree_map.iter().map(|(k, v)| (Yaml::String(k.clone()), to_yaml(v)));
            Yaml::Hash(BTreeMap::from_iter(iter))
        }
    }
}

pub fn output<T: Write>(w: &mut T, value: &Value) -> Result<(), serde_json::Error> {
    let mut out = String::new();
    YamlEmitter::new(&mut out).dump(&to_yaml(value)).unwrap_or_else(|_| unreachable!());
    write!(w, "{}", out).map_err(|e| serde_json::Error::IoError(e))
}
