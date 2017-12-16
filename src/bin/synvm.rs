extern crate clap;
extern crate rustacor;
extern crate byteorder;

use rustacor::assembler;
use rustacor::vm;

use clap::{App, Arg, ArgGroup};
use std::char;
use std::error::Error;
use std::fs::File;
use std::io::{Read, stdin};

fn run() -> Result<(), String> {
    let matches = App::new("synvm")
        .arg(Arg::with_name("binary")
            .index(1))
        .arg(Arg::with_name("asm")
            .long("asm")
            .value_name("asmfile")
            .takes_value(true))
        .arg(Arg::with_name("input_file")
            .short("f"))
        .arg(Arg::with_name("input_str")
            .short("i"))
        .group(ArgGroup::with_name("code").args(&["binary", "asm"]).required(true))
        .group(ArgGroup::with_name("input").args(&["input_file", "input_str"]))
        .get_matches_safe().map_err(|x| { x.description().to_string() })?;

    let mut vm = if let Some(file_name) = matches.value_of("binary") {
        let mut file = File::open(file_name).map_err(|_| "Unable to open input file")?;
        let vm = vm::VM::new_from_reader(&mut file);
        vm
    } else if let Some(asm_file_name) = matches.value_of("asm") {
        let mut asm_file = File::open(asm_file_name).map_err(|_| "Unable to open input asm file")?;
        let mut s = String::new();
        asm_file.read_to_string(&mut s).map_err(|_| "Unable to read asm input")?;

        let mut out = Vec::new();
        assembler::assemble(&mut out, &s).map_err(|e| format!("While assembling code: {}", match e {
            assembler::AssemblerError::LabelResolveError(ref s) => format!("Unknown label :{}", s),
            assembler::AssemblerError::ParserError(e) => format!("\n{}", e)
        }))?;

        let mut slc: &[u8] = &mut out;
        vm::VM::new_from_reader(&mut slc)
    } else { unreachable!() };

    if let Some(f) = matches.value_of("input_file") {
        let mut input_file = File::open(f).map_err(|_| "Unable to open input file")?;

        let mut s = String::new();
        input_file.read_to_string(&mut s).map_err(|_| "Unable to read input file")?;

        let mut char_iter = s.chars().collect::<Vec<_>>().into_iter();
        vm.set_input_callback(move || {
            char_iter.next().map_or(0, |x| x as u16)
        });
    } else if let Some(s) = matches.value_of("input_str") {
        let mut char_iter = s.chars().collect::<Vec<_>>().into_iter();
        vm.set_input_callback(move || {
            char_iter.next().map_or(0, |x| x as u16)
        });
    } else {
        vm.set_input_callback(|| {
            let x = stdin().bytes().next();
            let y = x.unwrap_or(Ok(0));
            let z = y.expect("Unable to read from stdin");
            z as u16
        });
    }
    vm.set_output_callback(|v| {
        print!("{}", char::from_u32(v as u32).expect("Cannot convert to char"));
    });
    vm.execute().map_err(|e| match e {
        vm::VMError::PopFromEmptyStack => "Popped from empty stack".to_string(),
        vm::VMError::UnknownInstruction(i) => format!("Unknown instruction {}", i),
        vm::VMError::OOBRegister(i) => format!("Unknown register access {}", i)
    })?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}