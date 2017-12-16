extern crate clap;
extern crate rustacor;

use rustacor::assembler;

use clap::*;
use std::fs::{File};
use std::io::{Read};

fn main() {
    let matches = App::new("synasm")
        .arg(Arg::with_name("output")
            .short("-o")
            .long("out")
            .takes_value(true)
            .value_name("FILE")
            .required(true))
        .arg(Arg::with_name("input")
            .required(true)
            .index(1))
        .get_matches();

    if let (Some(file_name), Some(output_name)) = (matches.value_of("input"), matches.value_of("output")) {
        let mut f = File::open(file_name).expect("Unable to open file");
        let mut src = String::new();
        f.read_to_string(&mut src).expect("Unable to read file");

        let mut o = File::create(output_name).expect("Unable to open output file");
        assembler::assemble(&mut o, &src).map_err(|e| format!("While assembling code: {}", match e {
            assembler::AssemblerError::LabelResolveError(ref s) => format!("Unknown label :{}", s),
            assembler::AssemblerError::ParserError(e) => format!("\n{}", e)
        })).unwrap();
    }
}