use std::io::Write;
use rustc_serialize;
use serde_json;
use serde_json::value::Value;

use self::yaml::Yaml;
use self::json::Json;

mod yaml;
mod json;

#[derive(Debug)]
pub enum OutputFormat {
    Json(Json),
    Yaml(Yaml),
}

pub trait Output {
   fn output<T: Write>(self, w: &mut T, value: &Value) -> Result<(), serde_json::Error>;
}

impl Output for OutputFormat {
   fn output<T: Write>(self, w: &mut T, value: &Value) -> Result<(), serde_json::Error> {
       match self {
           OutputFormat::Json(json) => json.output(w, value),
           OutputFormat::Yaml(yaml) => yaml.output(w, value),
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
                    "json" => Ok(OutputFormat::Json(Json::new(output_args))),
                    "yaml" => Ok(OutputFormat::Yaml(Yaml::new(output_args))),
                    "" => Ok(OutputFormat::Json(Json::new(output_args))),
                    _ => Err(d.error(format!("Unsupported output format \"{}\"", string).as_str())),
                }
            },
            _ => Err(d.error("Parsing failed name of output format")),
        }
    }
}
