use ::assembler::ProgramElement;
use ::instruction::{Instruction, Register, Parameter};

use std::str::{self};

use pest;
use pest::iterators::{Pair, Pairs};
use pest::inputs::{StrInput};
use pest::{Parser};

// cache busting
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("synasm.pest");

#[derive(Parser)]
#[grammar = "synasm.pest"]
struct AsmParser;


fn parse_param(p: Pair<Rule, StrInput>) -> Parameter {
    match p.as_rule() {
        Rule::int_literal => Parameter::Literal(p.as_str().parse::<u16>().unwrap()),
        Rule::hex_literal => Parameter::Literal(u16::from_str_radix(&p.as_str()[1..], 16).unwrap()),
        Rule::char_literal => Parameter::Literal(p.as_str().chars().nth(1).unwrap() as u16),
        Rule::reg_ref => Parameter::Register(parse_reg(p)),
        Rule::label_ref => Parameter::Label(p.as_str()[1..].to_string()),
        _ => panic!()
    }
}

fn parse_reg(p: Pair<Rule, StrInput>) -> Register {
    Register(p.into_span().as_str()[1..].parse::<u8>().unwrap())
}

fn parse_instruction(pair: Pair<Rule, StrInput>) -> Instruction {
    let rule = pair.as_rule();

    let mut inner = pair.into_inner();

    match rule {
        Rule::ins_halt => Instruction::Halt,
        Rule::ins_set => Instruction::Set(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_push => Instruction::Push(parse_param(inner.next().unwrap())),
        Rule::ins_pop => Instruction::Pop(parse_reg(inner.next().unwrap())),
        Rule::ins_eq => Instruction::Eq(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_gt => Instruction::Gt(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_jmp => Instruction::Jmp(parse_param(inner.next().unwrap())),
        Rule::ins_jt => Instruction::Jt(parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_jf => Instruction::Jf(parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_add => Instruction::Add(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_mult => Instruction::Mult(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_mod => Instruction::Mod(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_and => Instruction::And(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_or => Instruction::Or(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_not => Instruction::Not(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_rmem => Instruction::Rmem(parse_reg(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_wmem => Instruction::Wmem(parse_param(inner.next().unwrap()), parse_param(inner.next().unwrap())),
        Rule::ins_call => Instruction::Call(parse_param(inner.next().unwrap())),
        Rule::ins_ret => Instruction::Ret,
        Rule::ins_out => Instruction::Out(parse_param(inner.next().unwrap())),
        Rule::ins_in => Instruction::In(parse_reg(inner.next().unwrap())),
        Rule::ins_noop => Instruction::Noop,
        Rule::ins_dmp => Instruction::Dmp,
        _ => panic!()
    }
}

pub fn parse_elem(pair: Pair<Rule, StrInput>) -> ProgramElement {
    match pair.as_rule() {
        Rule::instruction => ProgramElement::Instruction(parse_instruction(pair.into_inner().next().unwrap())),
        Rule::label_def => {
            let s = pair.as_str();
            ProgramElement::Label(s[..(s.len()-1)].to_string())
        }
        _ => panic!()
    }
}

pub fn parse(src: &str) -> Result<Vec<ProgramElement>, pest::Error<Rule, StrInput>> {
    let pairs: Pairs<Rule, pest::inputs::StrInput> = AsmParser::parse_str(Rule::main, src)?;

    Ok(pairs.map(parse_elem).collect())
}