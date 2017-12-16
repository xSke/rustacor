use ::instruction::{Instruction, Parameter};

use std::collections::HashMap;
use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramElement {
    Label(String),
    Instruction(Instruction),
    Data(Vec<u16>)
}

impl ProgramElement {
    fn size(&self) -> u16 {
        match *self {
            ProgramElement::Label(_) => 0,
            ProgramElement::Instruction(ref instr) => instr.len() as u16,
            ProgramElement::Data(ref  v) => v.len() as u16
        }
    }
}

fn locate_labels(elems: &Vec<ProgramElement>) -> HashMap<String, u16> {
    let mut acc = 0u16;
    let mut map: HashMap<String, u16> = HashMap::new();

    for elem in elems {
        if let &ProgramElement::Label(ref s) = elem {
            map.insert(s.clone(), acc);
        }
        acc += elem.size();
    }

    return map;
}

fn reify_label(param: &Parameter, labels: &HashMap<String, u16>) -> Result<Parameter, String> {
    match *param {
        Parameter::Label(ref s) => match labels.get(s) {
            Some(adr) => Ok(Parameter::Literal(*adr)),
            None => Err(s.clone())
        }
        _ => Ok(param.clone())
    }
}

fn reify_labels(elems: &mut Vec<ProgramElement>, labels: &HashMap<String, u16>) -> Result<(), String> {
    for elem in elems {
        if let &mut ProgramElement::Instruction(ref mut instr) = elem {
            match instr {
                &mut Instruction::Set(_, ref mut b) => *b = reify_label(&b, labels)?,
                &mut Instruction::Push(ref mut a) => *a = reify_label(&a, labels)?,
                &mut Instruction::Eq(_, ref mut b, ref mut c) => {*b = reify_label(&b, labels)?; *c = reify_label(&c, labels)?},
                &mut Instruction::Gt(_, ref mut b, ref mut c) => {*b = reify_label(&b, labels)?; *c = reify_label(&c, labels)?},
                &mut Instruction::Jmp(ref mut a) => *a = reify_label(&a, labels)?,
                &mut Instruction::Jt(ref mut a, ref mut b) => {*a = reify_label(&a, labels)?; *b = reify_label(&b, labels)?},
                &mut Instruction::Jf(ref mut a, ref mut b) => {*a = reify_label(&a, labels)?; *b = reify_label(&b, labels)?},
                &mut Instruction::Add(_, ref mut b, ref mut c) => {*b = reify_label(&b, labels)?; *c = reify_label(&c, labels)?},
                &mut Instruction::Mult(_, ref mut b, ref mut c) => {*b = reify_label(&b, labels)?; *c = reify_label(&c, labels)?},
                &mut Instruction::Mod(_, ref mut b, ref mut c) => {*b = reify_label(&b, labels)?; *c = reify_label(&c, labels)?},
                &mut Instruction::And(_, ref mut b, ref mut c) => {*b = reify_label(&b, labels)?; *c = reify_label(&c, labels)?},
                &mut Instruction::Or(_, ref mut b, ref mut c) => {*b = reify_label(&b, labels)?; *c = reify_label(&c, labels)?},
                &mut Instruction::Not(_, ref mut b) => *b = reify_label(&b, labels)?,
                &mut Instruction::Rmem(_, ref mut b) => *b = reify_label(&b, labels)?,
                &mut Instruction::Wmem(ref mut a, ref mut b) => {*a = reify_label(&a, labels)?; *b = reify_label(&b, labels)?},
                &mut Instruction::Call(ref mut a) => *a = reify_label(&a, labels)?,
                &mut Instruction::Out(ref mut a) => *a = reify_label(&a, labels)?,
                _ => {}
            };
        }
    }
    Ok(())
}

fn write_program(out: &mut Write, prg: &Vec<ProgramElement>) {
    for elem in prg {
        match *elem {
            ProgramElement::Instruction(ref instr) => instr.write(out),
            ProgramElement::Data(ref d) => {
                for v in d {
                    out.write_u16::<LittleEndian>(*v).expect("Unable to write to file");
                }
            },
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum AssemblerError {
    ParserError(String),
    LabelResolveError(String)
}

pub fn assemble(out: &mut Write, src: &str) -> Result<(), AssemblerError> {
    let mut res = ::parser::parse(src).map_err(|e| AssemblerError::ParserError(e.to_string()))?;
    let labels = locate_labels(&res);
    reify_labels(&mut res, &labels).map_err(|x| AssemblerError::LabelResolveError(x))?;
    write_program(out, &res);
    Ok(())
}