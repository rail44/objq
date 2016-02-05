use std::io::Read;
use rustc_serialize;
use serde_json;
use serde_json::value::Value;

mod json;
mod yaml;
mod msgpack;

#[derive(Debug)]
pub enum InputFormat {
    Json(json::Json),
    Yaml(yaml::Yaml),
    MessagePack(msgpack::MessagePack),
}

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Yaml(yaml::Error),
    MessagePack(msgpack::Error),
}

pub trait Input {
    type Error;

    fn input<T: Read>(self, r: &mut T) -> Result<Value, Self::Error>;
}

impl Input for InputFormat {
    type Error = Error;

    fn input<T: Read>(self, r: &mut T) -> Result<Value, Self::Error> {
        match self {
            InputFormat::Json(json) => json.input(r).map_err(|e| Error::Json(e)),
            InputFormat::Yaml(yaml) => yaml.input(r).map_err(|e| Error::Yaml(e)),
            InputFormat::MessagePack(msgpack) => msgpack.input(r).map_err(|e| Error::MessagePack(e)),
        }
    }
}

impl rustc_serialize::Decodable for InputFormat {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<InputFormat, D::Error> {
        match d.read_str() {
            Ok(string) => match string.as_str() {
                "json" => Ok(InputFormat::Json(json::Json::new())),
                "yaml" => Ok(InputFormat::Yaml(yaml::Yaml::new())),
                "msgpack" => Ok(InputFormat::MessagePack(msgpack::MessagePack::new())),
                "" => Ok(InputFormat::Json(json::Json::new())),
                _ => Err(d.error(format!("Unsupported input format \"{}\"", string).as_str())),
            },
            _ => Err(d.error("Parsing failed name of input format")),
        }
    }
}
