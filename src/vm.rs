use ::instruction::{Instruction, Parameter, Register};
use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct VM<'a> {
    pc: u16,
    registers: [u16; 8],
    memory: [u16; 32768],
    stack: Vec<u16>,
    input_callback: Box<FnMut() -> u16 + 'a>,
    output_callback: Box<FnMut(u16) + 'a>
}

pub enum VMError {
    PopFromEmptyStack,
    UnknownInstruction(u16),
    OOBRegister(u16)
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        return VM {
            pc: 0,
            registers: [0; 8],
            memory: [0; 32768],
            stack: Vec::new(),
            input_callback: Box::new(|| 0),
            output_callback: Box::new(|_| {})
        };
    }

    pub fn new_from_reader(reader: &mut Read) -> Self {
        let mut vm = VM::new();

        let mut i = 0;
        while let Ok(v) = reader.read_u16::<LittleEndian>() {
            vm.memory[i] = v;
            i += 1;
        }

        return vm;
    }

    pub fn set_input_callback<F: 'a>(&mut self, f: F) where F: FnMut() -> u16 {
        self.input_callback = Box::new(f);
    }

    pub fn set_output_callback<F: 'a>(&mut self, f: F) where F: FnMut(u16) {
        self.output_callback = Box::new(f);
    }

    fn next_word(&mut self) -> u16 {
        let v = self.memory[self.pc as usize];
        self.pc += 1;
        return v;
    }

    fn load_instruction(&mut self) -> Result<Instruction, VMError> {
        let instr = self.next_word();
        match instr {
            0 => Ok(Instruction::Halt),
            1 => Ok(Instruction::Set(self.next_word().into(), self.next_word().into())),
            2 => Ok(Instruction::Push(self.next_word().into())),
            3 => Ok(Instruction::Pop(self.next_word().into())),
            4 => Ok(Instruction::Eq(self.next_word().into(), self.next_word().into(), self.next_word().into())),
            5 => Ok(Instruction::Gt(self.next_word().into(), self.next_word().into(), self.next_word().into())),
            6 => Ok(Instruction::Jmp(self.next_word().into())),
            7 => Ok(Instruction::Jt(self.next_word().into(), self.next_word().into())),
            8 => Ok(Instruction::Jf(self.next_word().into(), self.next_word().into())),
            9 => Ok(Instruction::Add(self.next_word().into(), self.next_word().into(), self.next_word().into())),
            10 => Ok(Instruction::Mult(self.next_word().into(), self.next_word().into(), self.next_word().into())),
            11 => Ok(Instruction::Mod(self.next_word().into(), self.next_word().into(), self.next_word().into())),
            12 => Ok(Instruction::And(self.next_word().into(), self.next_word().into(), self.next_word().into())),
            13 => Ok(Instruction::Or(self.next_word().into(), self.next_word().into(), self.next_word().into())),
            14 => Ok(Instruction::Not(self.next_word().into(), self.next_word().into())),
            15 => Ok(Instruction::Rmem(self.next_word().into(), self.next_word().into())),
            16 => Ok(Instruction::Wmem(self.next_word().into(), self.next_word().into())),
            17 => Ok(Instruction::Call(self.next_word().into())),
            18 => Ok(Instruction::Ret),
            19 => Ok(Instruction::Out(self.next_word().into())),
            20 => Ok(Instruction::In(self.next_word().into())),
            21 => Ok(Instruction::Noop),
            0xff => Ok(Instruction::Dmp),
            _ => Err(VMError::UnknownInstruction(instr))
        }
    }

    fn get_register(&self, reg: &Register) -> Result<u16, VMError> {
        if reg.0 >= 8 {return Err(VMError::OOBRegister(reg.0 as u16))}
        return Ok(self.registers[reg.0 as usize]);
    }

    fn set_register(&mut self, reg: &Register, v: u16) -> Result<(), VMError> {
        if reg.0 >= 8 {return Err(VMError::OOBRegister(reg.0 as u16))}
        self.registers[reg.0 as usize] = v;
        Ok(())
    }

    fn get_parameter(&self, param: &Parameter) -> Result<u16, VMError> {
        match *param {
            Parameter::Register(ref r) => self.get_register(r),
            Parameter::Literal(l) => Ok(l),
            _ => panic!("Unsupported parameter type {:?}", param)
        }
    }

    fn stack_push(&mut self, v: u16) {
        self.stack.push(v);
    }

    fn stack_pop(&mut self) -> Result<u16, VMError> {
        return match self.stack.pop() {
            Some(x) => Ok(x),
            None => Err(VMError::PopFromEmptyStack)
        };
    }

    fn evaluate(&mut self, instr: Instruction) -> Result<bool, VMError> {
        match instr {
            Instruction::Halt => return Ok(false),
            Instruction::Set(ref a, ref b) => {
                let v = self.get_parameter(b)?;
                self.set_register(a, v)?
            }
            Instruction::Push(ref a) => {
                let v = self.get_parameter(a)?;
                self.stack_push(v);
            }
            Instruction::Pop(ref a) => {
                let v = self.stack_pop()?;
                self.set_register(a, v)?;
            }
            Instruction::Eq(ref a, ref b, ref c) => {
                let v = self.get_parameter(b)? == self.get_parameter(c)?;
                self.set_register(a, if v { 1 } else { 0 })?;
            }
            Instruction::Gt(ref a, ref b, ref c) => {
                let v = self.get_parameter(b)? > self.get_parameter(c)?;
                self.set_register(a, if v { 1 } else { 0 })?;
            }
            Instruction::Jmp(ref a) => self.pc = self.get_parameter(a)?,
            Instruction::Jt(ref a, ref b) => {
                if self.get_parameter(a)? != 0 {
                    self.pc = self.get_parameter(b)?;
                }
            }
            Instruction::Jf(ref a, ref b) => {
                if self.get_parameter(a)? == 0 {
                    self.pc = self.get_parameter(b)?;
                }
            }
            Instruction::Add(ref a, ref b, ref c) => {
                let v = self.get_parameter(b)?.wrapping_add(self.get_parameter(c)?);
                self.set_register(a, v % 32768)?
            }
            Instruction::Mult(ref a, ref b, ref c) => {
                let v = self.get_parameter(b)?.wrapping_mul(self.get_parameter(c)?);
                self.set_register(a, v % 32768)?
            }
            Instruction::Mod(ref a, ref b, ref c) => {
                let v = self.get_parameter(b)? % self.get_parameter(c)?;
                self.set_register(a, v % 32768)?
            }
            Instruction::And(ref a, ref b, ref c) => {
                let v = self.get_parameter(b)? & self.get_parameter(c)?;
                self.set_register(a, v % 32768)?
            }
            Instruction::Or(ref a, ref b, ref c) => {
                let v = self.get_parameter(b)? | self.get_parameter(c)?;
                self.set_register(a, v % 32768)?
            }
            Instruction::Not(ref a, ref b) => {
                let v = !self.get_parameter(b)?;
                self.set_register(a, v % 32768)?
            }
            Instruction::Rmem(ref a, ref b) => {
                let v = self.memory[self.get_parameter(b)? as usize];
                self.set_register(a, v)?;
            }
            Instruction::Wmem(ref a, ref b) => {
                self.memory[self.get_parameter(a)? as usize] = self.get_parameter(b)?;
            }
            Instruction::Call(ref a) => {
                let pc = self.pc;
                self.stack_push(pc);
                self.pc = self.get_parameter(a)?;
            }
            Instruction::Ret => {
                self.pc = self.stack_pop()?;
            }
            Instruction::Out(ref a) => {
                let param = self.get_parameter(a)?;
                (self.output_callback)(param);
            }
            Instruction::In(ref a) => {
                let v = (self.input_callback)();
                self.set_register(a, v)?;
            },
            Instruction::Dmp => {
                println!("Registers: {:?}", self.registers);
                println!("Stack: {:?}", self.stack);
                println!("Memory (40xx): {:?}", &self.memory[0x4000..0x4100]);
                println!("Memory (410x): {:?}", &self.memory[0x4100..0x4110]);
                println!("Memory (50xx): {:?}", &self.memory[0x5000..0x5100]);
                println!("Memory (60xx): {:?}", &self.memory[0x6000..0x6100]);
                println!("-----");
            },
            Instruction::Noop => {}
        }
        return Ok(true);
    }

    fn step(&mut self) -> Result<bool, VMError> {
        let instr = self.load_instruction()?;
        return self.evaluate(instr);
    }

    pub fn execute(&mut self) -> Result<(), VMError> {
        while self.step()? {};
        Ok(())
    }
}