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

pub fn parse(src: &str) -> Result<Vec<ProgramElement>, (usize, usize)> {
    let pairs: Pairs<Rule, pest::inputs::StrInput> = match AsmParser::parse_str(Rule::main, src) {
        Err(e) => panic!("{}", e),
        Ok(v) => v
    };

    Ok(pairs.map(parse_elem).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = super::AsmParser::parse_str(super::Rule::main, "; KNOTHASH SYNACOR VM ASM
; STORES MAIN STATE IN 0x4000-0x40ff
; STORES DENSE HASH IN 0x4100-0x410f
jmp :main
xor:
    ; $6 = x & ~y
    not $6 $2
    and $6 $1 $6

    ; $7 = ~y & y
    not $7 $1
    and $7 $7 $2

    ; $0 = $6 | $7
    or $0 $6 $7
    ret");
        match x {
            Ok(_) => println!("{:?}", x),
            Err(e) => println!("{}", e)
        }
    }
}

//named!(comment<&[u8]>, preceded!(char!(';'), take_until_either_and_consume!("\r\n")));
//
//named!(astr1<String>, map!(many1!(none_of!("\r\n\t: ")), |x:Vec<char>|x.into_iter().collect()));
//
//named!(num<u16>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
//
//named!(char_lit<u16>, map!(delimited!(tag!("'"), none_of!("\r\n'"), tag!("'")), |x| x as u16));
//
//named!(hex_lit<u16>, map_res!(map_res!(preceded!(tag!("x"), recognize!(many_m_n!(1, 4, one_of!("0123456789abcdefABCDEF")))), str::from_utf8), |x|u16::from_str_radix(x, 16)));
//
//named!(register<Register>, do_parse!(
//    tag!("$") >>
//    r: map_opt!(one_of!("0123456789"), |x|char::to_digit(x, 10)) >>
//    (Register(r as u8))
//));
//
//named!(label_param<String>, do_parse!(
//    tag!(":") >>
//    l: astr1 >>
//    (l)
//));
//
//named!(parameter<Parameter>, alt!(
//    char_lit => {|x| Parameter::Literal(x)} |
//    hex_lit => {|x| Parameter::Literal(x)} |
//    register => {|x| Parameter::Register(x)} |
//    label_param => {|x| Parameter::Label(x)} |
//    num => {|x| Parameter::Literal(x)}
//));
//
//named!(instruction<Instruction>, ws!(alt!(
//    complete!(tag!("halt")) => {|_| Instruction::Halt } |
//    preceded!(complete!(tag!("set")), tuple!(register, parameter)) => {|(a, b)| Instruction::Set(a, b)} |
//    preceded!(complete!(tag!("push")), tuple!(parameter)) => {|a| Instruction::Push(a)} |
//    preceded!(complete!(tag!("pop")), tuple!(register)) => {|a| Instruction::Pop(a)} |
//    preceded!(complete!(tag!("eq")), tuple!(register, parameter, parameter)) => {|(a, b, c)| Instruction::Eq(a, b, c)} |
//    preceded!(complete!(tag!("gt")), tuple!(register, parameter, parameter)) => {|(a, b, c)| Instruction::Gt(a, b, c)} |
//    preceded!(complete!(tag!("jmp")), tuple!(parameter)) => {|a| Instruction::Jmp(a)} |
//    preceded!(complete!(tag!("jt")), tuple!(parameter, parameter)) => {|(a, b)| Instruction::Jt(a, b)} |
//    preceded!(complete!(tag!("jf")), tuple!(parameter, parameter)) => {|(a, b)| Instruction::Jf(a, b)} |
//    preceded!(complete!(tag!("add")), tuple!(register, parameter, parameter)) => {|(a, b, c)| Instruction::Add(a, b, c)} |
//    preceded!(complete!(tag!("mult")), tuple!(register, parameter, parameter)) => {|(a, b, c)| Instruction::Mult(a, b, c)} |
//    preceded!(complete!(tag!("mod")), tuple!(register, parameter, parameter)) => {|(a, b, c)| Instruction::Mod(a, b, c)} |
//    preceded!(complete!(tag!("and")), tuple!(register, parameter, parameter)) => {|(a, b, c)| Instruction::And(a, b, c)} |
//    preceded!(complete!(tag!("or")), tuple!(register, parameter, parameter)) => {|(a, b, c)| Instruction::Or(a, b, c)} |
//    preceded!(complete!(tag!("rmem")), tuple!(register, parameter)) => {|(a, b)| Instruction::Rmem(a, b)} |
//    preceded!(complete!(tag!("not")), tuple!(register, parameter)) => {|(a, b)| Instruction::Not(a, b)} |
//    preceded!(complete!(tag!("wmem")), tuple!(parameter, parameter)) => {|(a, b)| Instruction::Wmem(a, b)} |
//    complete!(tag!("ret")) => {|_| Instruction::Ret } |
//    preceded!(complete!(tag!("out")), tuple!(parameter)) => {|a| Instruction::Out(a)} |
//    preceded!(complete!(tag!("in")), tuple!(register)) => {|a| Instruction::In(a)} |
//    complete!(tag!("noop")) => {|_| Instruction::Noop }
//)));
//
//named!(label<String>, ws!(terminated!(astr1, tag!(":"))));
//
//// note: both subbranches eat trailing \n-s
//named!(element<ProgramElement>, preceded!(opt!(multispace), alt!(
//     complete!(label) => {|x| ProgramElement::Label(x)} |
//     instruction => {|x| ProgramElement::Instruction(x)}
//)));
//
//named!(elements<Vec<ProgramElement>>, ws!(do_parse!(
//    many0!(comment) >>
//    e: many0!(do_parse!(
//        e: element >>
//        many0!(comment) >>
//        (e)
//    )),
//    eof!()
//)));
//
//
//#[cfg(test)]
//mod tests {
//    use std::fmt::Debug;
//    use nom::*;
//
//    fn assert_parse<T: PartialEq + Debug>(res: IResult<&[u8], T>, exp: T, left: &[u8]) {
//        match res {
//            IResult::Done(i, o) => {
//                assert_eq!((i, o), (left, exp));
//            }
//            _ => panic!("assertion failed: parser errored: {:?}", res)
//        }
//    }
//
//    fn assert_incomplete<T: Debug>(res: IResult<&[u8], T>) {
//        match res {
//            IResult::Incomplete(_) => {}
//            _ => panic!("assertion failed: parser wasn't incomplete: {:?}", res)
//        }
//    }
//
//    fn assert_error<T: Debug>(res: IResult<&[u8], T>) {
//        match res {
//            IResult::Error(_) => {}
//            _ => panic!("assertion failed: parser did not error: {:?}", res)
//        }
//    }
//
//    #[test]
//    fn test_astr() {
//        assert_eq!(super::astr1(b"hello"), IResult::Done(&b""[..], "hello".to_string()));
//        assert_eq!(super::astr1(b"hello123"), IResult::Done(&b""[..], "hello123".to_string()));
//        assert_eq!(super::astr1(b"hello123 abcde"), IResult::Done(&b" abcde"[..], "hello123".to_string()));
//        assert_incomplete(super::astr1(b""));
//    }
//
//    #[test]
//    fn test_digit() {
//        assert_eq!(super::num(b"123"), IResult::Done(&b""[..], 123));
//        assert_eq!(super::num(b"456abc"), IResult::Done(&b"abc"[..], 456));
//        assert_eq!(super::num(b"456.6abc"), IResult::Done(&b".6abc"[..], 456));
//    }
//
//    #[test]
//    fn test_comment() {
//        assert_eq!(super::comment(b";test"), IResult::Done(&b""[..], &b"test"[..]));
//        assert_eq!(super::comment(b";test hello"), IResult::Done(&b""[..], &b"test hello"[..]));
//        assert_eq!(super::comment(b";test hello\nsomething"), IResult::Done(&b"\nsomething"[..], &b"test hello"[..]));
//    }
//
//    #[test]
//    fn test_charlit() {
//        assert_error(super::char_lit(b"A"));
//        assert_parse(super::char_lit(b"'a'"), 'a' as u16, b"");
//        assert_error(super::char_lit(b"'ab'"));
//        assert_error(super::char_lit(b"''"));
//        assert_incomplete(super::char_lit(b"'b"));
//    }
//
//    #[test]
//    fn test_hexlit() {
//        assert_incomplete(super::hex_lit(b"x"));
//        assert_parse(super::hex_lit(b"xA"), 0xA, b"");
//        assert_parse(super::hex_lit(b"xAB"), 0xAB, b"");
//        assert_parse(super::hex_lit(b"xABC"), 0xABC, b"");
//        assert_parse(super::hex_lit(b"xABCD"), 0xABCD, b"");
//        assert_parse(super::hex_lit(b"xABCDE"), 0xABCD, b"E");
//        assert_parse(super::hex_lit(b"x4000"), 0x4000, b"");
//    }
//
//    #[test]
//    fn test_reg() {
//        assert_incomplete(super::register(b"$"));
//        assert_parse(super::register(b"$1"), super::Register(1), b"");
//        assert_error(super::register(b"$a"));
//        assert_parse(super::register(b"$10"), super::Register(1), b"0");
//        assert_parse(super::register(b"$1a"), super::Register(1), b"a");
//    }
//
//    #[test]
//    fn test_label_param() {
//        assert_incomplete(super::label_param(b":"));
//        assert_parse(super::label_param(b":hi"), "hi".to_string(), b"");
//        assert_parse(super::label_param(b":hihello\ntest"), "hihello".to_string(), b"\ntest");
//        assert_parse(super::label_param(b":hi_hello\ntest"), "hi_hello".to_string(), b"\ntest");
//        assert_parse(super::label_param(b":hi.hello\ntest"), "hi.hello".to_string(), b"\ntest");
//    }
//
//    #[test]
//    fn test_param() {
//        assert_parse(super::parameter(b"123q"), super::Parameter::Literal(123), b"q");
//        assert_parse(super::parameter(b"$5abc"), super::Parameter::Register(super::Register(5)), b"abc");
//        assert_parse(super::parameter(b":test"), super::Parameter::Label("test".to_string()), b"");
//        assert_parse(super::parameter(b":test\nhi"), super::Parameter::Label("test".to_string()), b"\nhi");
//        assert_parse(super::parameter(b"x4040"), super::Parameter::Literal(0x4040), b"");
//        assert_incomplete(super::parameter(b":"));
//        assert_incomplete(super::parameter(b"$"));
//        assert_incomplete(super::parameter(b""));
//    }
//
//    #[test]
//    fn test_instr() {
//        assert_parse(super::instruction(b"add $0 1 2"), super::Instruction::Add(super::Register(0), super::Parameter::Literal(1), super::Parameter::Literal(2)), b"");
//        assert_parse(super::instruction(b"or $6 3 :test abcd"), super::Instruction::Or(super::Register(6), super::Parameter::Literal(3), super::Parameter::Label("test".to_string())), b"abcd");
//        assert_incomplete(super::instruction(b"add $0 $2"));
//        assert_incomplete(super::instruction(b"set"));
//        assert_parse(super::instruction(b"haltnowpls"), super::Instruction::Halt, b"nowpls");
//    }
//
//    #[test]
//    fn test_elem() {
//        assert_parse(super::element(b"add $0 1 2"), super::ProgramElement::Instruction(super::Instruction::Add(super::Register(0), super::Parameter::Literal(1), super::Parameter::Literal(2))), b"");
//        assert_parse(super::element(b"add $0 1 2\nlabel:"), super::ProgramElement::Instruction(super::Instruction::Add(super::Register(0), super::Parameter::Literal(1), super::Parameter::Literal(2))), b"label:");
//        assert_parse(super::element(b"test:\nhalt"), super::ProgramElement::Label("test".to_string()), b"halt");
//        assert_parse(super::element(b"noop:\nabcd"), super::ProgramElement::Label("noop".to_string()), b"abcd");
//        assert_parse(super::element(b"noop\nabcd"), super::ProgramElement::Instruction(super::Instruction::Noop), b"abcd");
//    }
//
//    #[test]
//    fn test_elems() {
//        assert_parse(super::elements(b"  add $0 1 2 ;Add two numbers  \n test:;Label   \nhalt;Stop it"), vec![
//            super::ProgramElement::Instruction(super::Instruction::Add(super::Register(0), super::Parameter::Literal(1), super::Parameter::Literal(2))),
//            super::ProgramElement::Label("test".to_string()),
//            super::ProgramElement::Instruction(super::Instruction::Halt)
//        ], b"");
//        assert_error(super::elements(b"add $0"));
//        assert_error(super::elements(b"ste $0 $2"));
//    }
//}