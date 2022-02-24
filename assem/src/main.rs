#![warn(clippy::all)]

use std::env;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::str::FromStr;

use grumpy::isa::*;
use grumpy::assemble::*;
use grumpy::*;

fn main() -> io::Result<()> {
    // Read input file (command line argument at index 1).
    let args: Vec<String> = env::args().collect();
    let file = OpenOptions::new().read(true).open(&args[1]).expect("Error getting input");
    let reader = BufReader::new(file);

    let mut inp: Vec<isa::PInstr> = Vec::new();
    for line in reader.lines(){
        inp.push(isa::PInstr::from_str(&line?)?);
    }
    // Convert file contents to vector of (labeled) instructions.
    let mut assembled_inp: Vec<Instr> = Vec::new();
    match assemble::assemble(&inp){
        Ok(T) => assembled_inp = T,
        Err(E) => std::process::exit(1),
    }

    let mut temp = &mut args[1].chars();
    temp.next_back();
    temp.next_back();
    let v = temp.as_str();

    let mut buffer = OpenOptions::new().write(true).create(true).open(v.to_owned() + ".o").expect("Error creating output file");


    // Resolve labels, converting the vector of labeled instructions
    // to a vector of assembled instructions.
    let mut pc: u32 = 0;
    for (count, i) in assembled_inp.iter().enumerate(){
        if count == assembled_inp.len() - 1{
            match i{
                Instr::Push(i) => match *i{
                    Val::Vloc(u) => pc = u,
                    _ => (),
                },
                _ => ()
            }

        }
    }
    assembled_inp.pop();

    let pc_bites = pc.to_be_bytes();
    buffer.write(&pc_bites).unwrap();
    for i in assembled_inp{
        let data = Instr::to_bytes(&i);
        buffer.write(&data).unwrap();
    }

    std::process::exit(0);
}
