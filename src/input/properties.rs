use std::io;
pub use super::Value;

#[derive(Debug)]
pub struct Properties;

impl Properties {
    pub fn new() -> Properties {
        Properties
    }
}

peg! grammer(r#"
use std::collections::BTreeMap;
use std::iter::FromIterator;
use super::{Value};

linebreak = ("\n")+
whitespace = (" ")*

#[pub]
parse -> Value
    = line_vec:line**linebreak linebreak? {
        Value::Object(
            BTreeMap::from_iter(
                line_vec.into_iter().filter(|opt| opt.is_some()).map(|some| some.unwrap())
            )
        )
    }

line -> Option<(String, Value)>
    = k:key whitespace "=" whitespace v:value { Some((k, v)) }
    / "\#" [^\n]+ { None }

key -> String
    = [^ =\n]+ { match_str.to_owned() }

value -> Value
    = [^\n]+ { Value::String(match_str.to_owned()) }
"#);

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(grammer::ParseError),
}

impl super::Input for Properties {
    type Error = Error;

    fn input<T: io::Read>(self, r: &mut T) -> Result<Value, Self::Error> {
        let mut in_str = String::new();
        try!(r.read_to_string(&mut in_str).map_err(|e| Error::Io(e)));
        grammer::parse(in_str.as_str()).map_err(|e| Error::Parse(e))
    }
}
