use std::io::Write;
use ::byteorder::{LittleEndian, WriteBytesExt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register(pub u8);

impl Register {
    fn to_word(&self) -> u16 {
        return 32768 + self.0 as u16;
    }
}

impl From<u16> for Register {
    fn from(p: u16) -> Self {
        return Register(p as u8);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Parameter {
    Literal(u16),
    Register(Register),
    Label(String)
}

impl Parameter {
    fn to_word(&self) -> u16 {
        match *self {
            Parameter::Literal(x) => x,
            Parameter::Register(ref x) => x.to_word(),
            Parameter::Label(_) => panic!("Cannot byteify a label reference - reify first")
        }
    }
}

impl From<u16> for Parameter {
    fn from(p: u16) -> Self {
        if p <= 32767 {
            return Parameter::Literal(p);
        } else {
            return Parameter::Register(p.into());
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Halt,
    Set(Register, Parameter),
    Push(Parameter),
    Pop(Register),
    Eq(Register, Parameter, Parameter),
    Gt(Register, Parameter, Parameter),
    Jmp(Parameter),
    Jt(Parameter, Parameter),
    Jf(Parameter, Parameter),
    Add(Register, Parameter, Parameter),
    Mult(Register, Parameter, Parameter),
    Mod(Register, Parameter, Parameter),
    And(Register, Parameter, Parameter),
    Or(Register, Parameter, Parameter),
    Not(Register, Parameter),
    Rmem(Register, Parameter),
    Wmem(Parameter, Parameter),
    Call(Parameter),
    Ret,
    Out(Parameter),
    In(Register),
    Noop,
    Dmp
}


impl Instruction {
    pub fn len_by_idx(idx: u16) -> u16 {
        match idx {
            0 => 1,
            1 => 3,
            2 => 2,
            3 => 2,
            4 => 4,
            5 => 4,
            6 => 2,
            7 => 3,
            8 => 3,
            9 => 4,
            10 => 4,
            11 => 4,
            12 => 4,
            13 => 4,
            14 => 3,
            15 => 3,
            16 => 3,
            17 => 2,
            18 => 1,
            19 => 2,
            20 => 2,
            21 => 1,
            0xff => 1,
            _ => panic!("Unknown instruction")
        }
    }

    pub fn len(&self) -> u16 {
        return Instruction::len_by_idx(self.idx());
    }
    fn idx(&self) -> u16 {
        match *self {
            Instruction::Halt => 0,
            Instruction::Set(_, _) => 1,
            Instruction::Push(_) => 2,
            Instruction::Pop(_) => 3,
            Instruction::Eq(_, _, _) => 4,
            Instruction::Gt(_, _, _) => 5,
            Instruction::Jmp(_) => 6,
            Instruction::Jt(_, _) => 7,
            Instruction::Jf(_, _) => 8,
            Instruction::Add(_, _, _) => 9,
            Instruction::Mult(_, _, _) => 10,
            Instruction::Mod(_, _, _) => 11,
            Instruction::And(_, _, _) => 12,
            Instruction::Or(_, _, _) => 13,
            Instruction::Not(_, _) => 14,
            Instruction::Rmem(_, _) => 15,
            Instruction::Wmem(_, _) => 16,
            Instruction::Call(_) => 17,
            Instruction::Ret => 18,
            Instruction::Out(_) => 19,
            Instruction::In(_) => 20,
            Instruction::Noop => 21,
            Instruction::Dmp => 0xff
        }
    }

    pub fn write(&self, out: &mut Write) {
        let mut buf = [0u16; 4];
        buf[0] = self.idx() as u16;
        match *self {
            Instruction::Set(ref a, ref b) => {buf[1] = a.to_word(); buf[2] = b.to_word();},
            Instruction::Push(ref a) => buf[1] = a.to_word(),
            Instruction::Pop(ref a) => buf[1] = a.to_word(),
            Instruction::Eq(ref a, ref b, ref c) => {buf[1] = a.to_word(); buf[2] = b.to_word(); buf[3] = c.to_word()},
            Instruction::Gt(ref a, ref b, ref c) => {buf[1] = a.to_word(); buf[2] = b.to_word(); buf[3] = c.to_word()},
            Instruction::Jmp(ref a) => buf[1] = a.to_word(),
            Instruction::Jt(ref a, ref b) => {buf[1] = a.to_word(); buf[2] = b.to_word();},
            Instruction::Jf(ref a, ref b) => {buf[1] = a.to_word(); buf[2] = b.to_word();},
            Instruction::Add(ref a, ref b, ref c) => {buf[1] = a.to_word(); buf[2] = b.to_word(); buf[3] = c.to_word()},
            Instruction::Mult(ref a, ref b, ref c) => {buf[1] = a.to_word(); buf[2] = b.to_word(); buf[3] = c.to_word()},
            Instruction::Mod(ref a, ref b, ref c) => {buf[1] = a.to_word(); buf[2] = b.to_word(); buf[3] = c.to_word()},
            Instruction::And(ref a, ref b, ref c) => {buf[1] = a.to_word(); buf[2] = b.to_word(); buf[3] = c.to_word()},
            Instruction::Or(ref a, ref b, ref c) => {buf[1] = a.to_word(); buf[2] = b.to_word(); buf[3] = c.to_word()},
            Instruction::Not(ref a, ref b) => {buf[1] = a.to_word(); buf[2] = b.to_word();},
            Instruction::Rmem(ref a, ref b) => {buf[1] = a.to_word(); buf[2] = b.to_word();},
            Instruction::Wmem(ref a, ref b) => {buf[1] = a.to_word(); buf[2] = b.to_word();},
            Instruction::Call(ref a) => buf[1] = a.to_word(),
            Instruction::Out(ref a) => buf[1] = a.to_word(),
            Instruction::In(ref a) => buf[1] = a.to_word(),
            _ => {}
        }
        for b in &buf[0..(self.len() as usize)] {
            out.write_u16::<LittleEndian>(*b).expect("Unable to write to output");
        }
    }
}