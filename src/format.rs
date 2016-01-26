use std::io::{Read, Write};
use rustc_serialize;
use serde_json;
use serde_json::value::Value;

#[derive(Debug)]
pub enum InputFormat {
    Json,
}

#[derive(Debug)]
pub enum OutputFormat {
    Json,
}

impl InputFormat {
   pub fn input<T: Read>(self, r: &mut T) -> Result<Value, serde_json::Error> {
       match self {
           InputFormat::Json => serde_json::from_reader(r),
       }
   }
}

impl OutputFormat {
   pub fn output<T: Write>(self, w: &mut T, value: &Value) -> Result<(), serde_json::Error> {
       match self {
           OutputFormat::Json => serde_json::to_writer_pretty(w, value),
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

impl rustc_serialize::Decodable for OutputFormat {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<OutputFormat, D::Error> {
        match d.read_str() {
            Ok(string) => match string.as_str() {
                "json" => Ok(OutputFormat::Json),
                "" => Ok(OutputFormat::Json),
                _ => Err(d.error(format!("Unsupported output format \"{}\"", string).as_str())),
            },
            _ => Err(d.error("Parsing failed name of output format")),
        }
    }
}
