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
    -o <format>    (json | yaml | msgpack)
    -q <query>     Query for data, with format like '.foo.bar[0]'
    -h, --help     Display this message
", flag_i: InputFormat, flag_o: OutputFormat, flag_q: Option<Query>);

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let obj: Value = args.flag_i.input(&mut io::stdin()).unwrap_or_else(|e| {
        panic!("{:?}", e);
    });

    let value = match args.flag_q {
        Some(query) => match obj.query(&query) {
            Some(value) => value,
            None => {
                panic!("query \"{:?}\" does not match with given object", query);
            },
        },
        None => &obj,
    };

    match args.flag_o.output(&mut io::stdout(), value) {
        Ok(_) => {},
        Err(_) => {
            panic!("");
        },
    }
}
