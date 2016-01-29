use std::io::Read;
use rustc_serialize;
use serde_json;
use serde_json::value::Value;

#[derive(Debug)]
pub enum InputFormat {
    Json,
}

impl InputFormat {
   pub fn input<T: Read>(self, r: &mut T) -> Result<Value, serde_json::Error> {
       match self {
           InputFormat::Json => serde_json::from_reader(r),
       }
   }
}

impl rustc_serialize::Decodable for InputFormat {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<InputFormat, D::Error> {
        match d.read_str() {
            Ok(string) => match string.as_str() {
                "json" => Ok(InputFormat::Json),
                "" => Ok(InputFormat::Json),
                _ => Err(d.error(format!("Unsupported input format \"{}\"", string).as_str())),
            },
            _ => Err(d.error("Parsing failed name of input format")),
        }
    }
}
