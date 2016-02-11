use std::io::Write;
use rustc_serialize;

pub use super::Value;

mod yaml;
mod json;
mod msgpack;

#[derive(Debug)]
pub enum OutputFormat {
    Json(json::Json),
    Yaml(yaml::Yaml),
    MessagePack(msgpack::MessagePack),
}

pub enum Error {
    Json(json::Error),
    Yaml(yaml::Error),
    MessagePack(msgpack::Error),
}

pub trait Output {
    type Error;
    fn output<T: Write>(self, w: &mut T, value: &Value) -> Result<(), Self::Error>;
}

impl Output for OutputFormat {
    type Error = Error;
    fn output<T: Write>(self, w: &mut T, value: &Value) -> Result<(), Self::Error> {
        match self {
            OutputFormat::Json(json) => json.output(w, value).map_err(|e| Error::Json(e)),
            OutputFormat::Yaml(yaml) => yaml.output(w, value).map_err(|e| Error::Yaml(e)),
            OutputFormat::MessagePack(msgpack) => msgpack.output(w, value).map_err(|e| Error::MessagePack(e)),
        }
    }
}

impl rustc_serialize::Decodable for OutputFormat {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<OutputFormat, D::Error> {
        match d.read_str() {
            Ok(string) => {
                let mut output_args: Vec<&str> = string.split(':').collect();
                output_args.reverse();
                match output_args.pop().unwrap() {
                    "json" => Ok(OutputFormat::Json(json::Json::new(output_args))),
                    "yaml" => Ok(OutputFormat::Yaml(yaml::Yaml::new(output_args))),
                    "msgpack" => Ok(OutputFormat::MessagePack(msgpack::MessagePack::new(output_args))),
                    "" => Ok(OutputFormat::Json(json::Json::new(output_args))),
                    _ => Err(d.error(format!("Unsupported output format \"{}\"", string).as_str())),
                }
            },
            _ => Err(d.error("Parsing failed name of output format")),
        }
    }
}
