#![feature(custom_derive, plugin)]
#![plugin(serde_macros, docopt_macros, peg_syntax_ext)]

extern crate serde;
extern crate serde_json;
extern crate docopt;
extern crate rustc_serialize;

use std::io;
use std::process;
use serde_json::value::Value;
use query::{Queryable, Query};
use format::InputFormat;
use output::OutputFormat;

mod query;
mod format;
mod output;

docopt!(Args derive Debug, "
objq

Usage:
  objq [--input=<format>] [--output=<format>] [--query=<query>]

Options:
  --query=<query>
", flag_input: InputFormat, flag_output: OutputFormat, flag_query: Option<Query>);


fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let obj: Value = args.flag_input.input(&mut io::stdin()).unwrap_or_else(|e| {
        println!("{:?}", e);
        process::exit(1);
    });

    let value = match args.flag_query {
        Some(query) => match obj.query(&query) {
            Some(value) => value,
            None => {
                println!("query \"{:?}\" does not match with given object", query);
                process::exit(1);
            },
        },
        None => &obj,
    };
    match args.flag_output.output(&mut io::stdout(), value) {
        Ok(_) => {},
        Err(_) => {
            process::exit(1);
        },
    }
}
