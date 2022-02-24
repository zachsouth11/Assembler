use crate::isa::{*, Instr::*, PInstr::*, Val::*};
use std::collections::HashMap;
/// Translate an assembly program to an equivalent bytecode program.
pub fn assemble(pinstrs : &[PInstr]) -> Result<Vec<Instr>, String> {
    let mut assembled_inp : Vec<Instr> = Vec::new();
    let mut pc: u32 = 0;
    let mut labels = HashMap::<String, u32>::new();
    let mut is_label = true;

    for i in pinstrs{
        match i {
            PInstr::PLabel(t) => is_label = true,
            _ => is_label = false,
        }
        if is_label{
            let string: &str = &i.to_string();
            let last_off: &str = &string[..string.len() - 1];
            labels.insert(last_off.to_string(), pc);
        }
        else{
            pc = pc + 1;
        }
    }

    for i in pinstrs {
        match i{
            PPush(t) => {
                match labels.get(t){
                    Some(K) => assembled_inp.push(Instr::Push(Val::Vloc(*K))),
                    None => (),
                }
            }
            PI(s) => assembled_inp.push(*s),
            _ => ()
        }
    }
    let count: u32 = pc;
    assembled_inp.push(Instr::Push(Val::Vloc(count)));
    Ok(assembled_inp)
}
