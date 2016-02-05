use std::io::Read;
use serde_json;
use serde_json::value::Value;

#[derive(Debug)]
pub struct Json;

impl Json {
    pub fn new() -> Json {
        Json
    }
}

pub type Error = serde_json::Error;

impl super::Input for Json {
    type Error = Error;

    fn input<T: Read>(self, r: &mut T) -> Result<Value, Self::Error> {
        serde_json::from_reader(r)
    }
}
