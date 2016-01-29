use std::io::Write;
use serde_json;
use serde_json::Value;

#[derive(Debug)]
pub struct Json {
    pretty: bool,
}

impl Json {
    pub fn new(op: Vec<&str>) -> Json {
        Json { pretty: op.contains(&"pretty") }
    }
}

impl super::Output for Json {
   fn output<T: Write>(self, w: &mut T, value: &Value) -> Result<(), serde_json::Error> {
       match self {
           Json { pretty: b } => match b {
               true => serde_json::to_writer_pretty(w, value),
               false => serde_json::to_writer(w, value),
           }
       }
   }
}
