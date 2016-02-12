use std::io::Read;
use rustc_serialize;
use serde_json;

pub use super::Value;

mod json;
mod yaml;
mod msgpack;
mod ini;
mod properties;

#[derive(Debug)]
pub enum InputFormat {
    Json(json::Json),
    Yaml(yaml::Yaml),
    MessagePack(msgpack::MessagePack),
    Ini(ini::Ini),
    Properties(properties::Properties),
}

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    Yaml(yaml::Error),
    MessagePack(msgpack::Error),
    Ini(ini::Error),
    Properties(properties::Error),
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
            InputFormat::Ini(ini) => ini.input(r).map_err(|e| Error::Ini(e)),
            InputFormat::Properties(properties) => properties.input(r).map_err(|e| Error::Properties(e)),
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
                "ini" => Ok(InputFormat::Ini(ini::Ini::new())),
                "properties" => Ok(InputFormat::Properties(properties::Properties::new())),
                "" => Ok(InputFormat::Json(json::Json::new())),
                _ => Err(d.error(format!("Unsupported input format \"{}\"", string).as_str())),
            },
            _ => Err(d.error("Parsing failed name of input format")),
        }
    }
}
