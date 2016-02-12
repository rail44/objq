#![feature(custom_derive, plugin, slice_patterns)]
#![plugin(serde_macros, docopt_macros, peg_syntax_ext)]

extern crate serde;
extern crate serde_json;
extern crate docopt;
extern crate rustc_serialize;
extern crate yaml_rust;
extern crate rmp_serde;

pub use serde_json::value::Value;

use std::io;
use std::fs::File;
use query::{Queryable, Query};
use input::{InputFormat, Input};
use output::{OutputFormat, Output};

mod query;
mod input;
mod output;

docopt!(Args derive Debug, "
Converter/Querier for data format

Usage: objq [options]
       objq (-h | --help)

Options:
    -i <format>    (json | yaml | msgpack | ini | properties)
    -o <format>    (json | json:pretty | yaml | msgpack)
    -q <query>     Query for data, with format like '.foo.bar[0]'
    -f <file>      Read from file instead of STDIN
    -h, --help     Display this message
", flag_i: InputFormat, flag_o: OutputFormat, flag_q: Option<Query>);

#[derive(Debug)]
enum Error {
    File(io::Error),
    Input(input::Error),
    Query,
    Output(output::Error),
}

fn cli(args: Args) -> Result<(), Error> {
    let value = match args.flag_f.as_str() {
        "" => try!(args.flag_i.input(&mut io::stdin()).map_err(|e| Error::Input(e))),
        file_name => {
            let mut file = try!(File::open(file_name).map_err(|e| Error::File(e)));
            try!(args.flag_i.input(&mut file).map_err(|e| Error::Input(e)))
        },
    };
    let queried = match args.flag_q {
        Some(query) => match value.query(&query) {
            Some(queried) => queried,
            None => return Err(Error::Query),
        },
        None => &value,
    };
    args.flag_o.output(&mut io::stdout(), queried).map_err(|e| Error::Output(e))
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());;
    cli(args).unwrap_or_else(|e| panic!("{:?}", e))
}
