use self::{Binop::*, Instr::*, PInstr::*, Unop::*, Val::*};
use crate::{ParseError, ToBytes};
use std::fmt::{self, Display};
use std::str::FromStr;

/// Heap addresses.
pub type Address = usize;

/// GrumpyVM values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Val {
    // Value types that may appear in GrumpyVM programs:
    /// The unit value.
    Vunit,
    /// 32-bit signed integers.
    Vi32(i32),
    /// Booleans.
    Vbool(bool),
    /// Stack or instruction locations.
    Vloc(u32),
    /// The undefined value.
    Vundef,

    // Value types that are used internally by the language
    // implementation, and may not appear in GrumpyVM programs:
    /// Metadata for heap objects that span multiple values.
    Vsize(i32),
    /// Pointers to heap locations.
    Vaddr(Address),
}

/// GrumpyVM native instructions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instr {
    /// Push(v): Push value v onto the stack.
    Push(Val),
    /// Pop a value from the stack, discarding it.
    Pop,
    /// Peek(i): Push onto the stack the ith value from the top.
    Peek(u32),
    /// Unary(u): Apply u to the top value on the stack.
    Unary(Unop),
    /// Binary(b): Apply b to the top two values on the stack,
    /// replacing them with the result.
    Binary(Binop),
    /// Swap the top two values.
    Swap,
    /// Allocate an array on the heap.
    Alloc,
    /// Write to a heap-allocated array.
    Set,
    /// Read from a heap-allocated array.
    Get,
    /// Var(i): Get the value at stack position fp+i.
    Var(u32),
    /// Store(i): Store a value at stack position fp+i.
    Store(u32),
    /// SetFrame(i): Set fp = s.stack.len() - i.
    SetFrame(u32),
    /// Function call.
    Call,
    /// Function return.
    Ret,
    /// Conditional jump.
    Branch,
    /// Halt the machine.
    Halt,
}

/// Program labels.
pub type Label = String;

/// Pseudo-instructions, extending native instructions with support
/// for labels. GrumpyVM cannot execute these directly -- they must
/// first be translated by the assembler to native instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum PInstr {
    /// Label the next instruction.
    PLabel(Label),
    /// Push a label onto the stack.
    PPush(Label),
    /// Native machine instruction.
    PI(Instr),
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unop {
    /// Boolean negation.
    Neg,
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Binop {
    /// i32 addition.
    Add,
    /// i32 multiplication.
    Mul,
    /// i32 subtraction.
    Sub,
    /// i32 division (raises an error on divide by zero).
    Div,
    /// Returns true if one i32 is less than another, otherwise false.
    Lt,
    /// Returns true if one i32 is equal another, otherwise false.
    Eq,
}

////////////////////////////////////////////////////////////////////////
// Display trait implementations
////////////////////////////////////////////////////////////////////////

impl Display for Unop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Neg => write!(f, "neg")
        }
    }
}

impl Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Add => write!(f, "+"),
            Mul => write!(f, "*"),
            Sub => write!(f, "-"),
            Div => write!(f, "/"),
            Lt  => write!(f, "<"),
            Eq  => write!(f, "=="),
        }
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Vunit    => write!(f, "tt"),
            Vi32(i)  => write!(f, "{}", i),
            Vbool(b) => write!(f, "{}", b),
            Vloc(u)  => write!(f, "{}", u),
            Vundef   => write!(f, "undef"),
            _ => Err(fmt::Error)
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Push(v)     => write!(f, "push {}", v),
            Pop         => write!(f, "pop"),
            Peek(u)     => write!(f, "peek {}", u),
            Unary(u)    => write!(f, "unary {}", u),
            Binary(b)   => write!(f, "binary {}", b),
            Swap        => write!(f, "swap"),
            Alloc       => write!(f, "alloc"),
            Set         => write!(f, "set"),
            Get         => write!(f, "get"),
            Var(u)      => write!(f, "var {}", u),
            Store(u)    => write!(f, "store {}", u),
            SetFrame(u) => write!(f, "setframe {}", u),
            Call        => write!(f, "call"),
            Ret         => write!(f, "ret"),
            Branch      => write!(f, "branch"),
            Halt        => write!(f, "halt"),
        }
    }
}

impl Display for PInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PLabel(lbl) => write!(f, "{}:", lbl),
            PPush(lbl)  => write!(f, "push {}", lbl),
            PI(instr)   => write!(f, "{}", instr)
        }
    }
}

////////////////////////////////////////////////////////////////////////
// FromStr trait implementations
////////////////////////////////////////////////////////////////////////

impl FromStr for Unop {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "neg" => Ok(Neg),
            _ => Err(ParseError("Unop Parse Error".to_string()))
        }
    }
}

impl FromStr for Binop {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "+" => Ok(Add),
            "*" => Ok(Mul),
            "-" => Ok(Sub),
            "/" => Ok(Div),
            "<" => Ok(Lt),
            "==" => Ok(Eq),
            _ => Err(ParseError("Binop Parse Error".to_string()))
        }
    }
}

impl FromStr for Val {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "tt" => Ok(Vunit),
            "undef" => Ok(Vundef),
            "true" => Ok(Vbool(true)),
            "false" => Ok(Vbool(false)),
            _ => match s.parse::<i32>() {
                Ok(T) => Ok(Vi32(T)),
                Err(T) => match s.parse::<u32>(){
                    Ok(E) => Ok(Vloc(E)),
                    Err(E) => Err(ParseError("Val Parse Error".to_string()))
                }
            }

        }
    }
}

impl FromStr for Instr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace();
        let split : Vec<&str> = split.collect();
        match split[0] {
            "push" => Ok(Push(Val::from_str(split[1])?)),
            "pop" => Ok(Pop),
            "peek" => Ok(Peek(split[1].parse::<u32>()?)),
            "unary" => Ok(Unary(Unop::from_str(split[1]).unwrap())),
            "binary" => Ok(Binary(Binop::from_str(split[1]).unwrap())),
            "swap" => Ok(Swap),
            "alloc" => Ok(Alloc),
            "set" => Ok(Set),
            "get" => Ok(Get),
            "var" => Ok(Var(split[1].parse::<u32>()?)),
            "store" => Ok(Store(split[1].parse::<u32>()?)),
            "setframe" => Ok(SetFrame(split[1].parse::<u32>()?)),
            "call" => Ok(Call),
            "ret" => Ok(Ret),
            "branch" => Ok(Branch),
            "halt" => Ok(Halt),
            _ => Err(ParseError("Instr Parse Error".to_string()))
        }
    }
}

fn parse_label(s: &str) -> Result<Label, ParseError> {
    type Err = ParseError;

    let split = s.split_whitespace();
    let split : Vec<&str> = split.collect();

    if split.len() != 1 {
        return Err(ParseError("Parse Label Error, Too little or too many inputs".to_string()))
    }

    for (i, letter) in s.chars().enumerate(){
        if i == 0 && letter != 'L'{
            if letter != '_' || s.chars().nth(1).unwrap() != 'L'{
                return Err(ParseError("ParseLabel Error".to_string()));
            }
        }
        if i == s.len()-1 && letter == ':'{
            let mut tmp = s.chars();
            tmp.next_back();
            return Ok(tmp.as_str().to_string());
        }
        if i != 0 && !letter.is_ascii_alphanumeric(){
            return Err(ParseError("ParseLabel Error".to_string()));
        }
    }
    return Ok(s.to_string());
}

impl FromStr for PInstr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace();
        let split : Vec<&str> = split.collect();

        match split[0] {
            "push" => match parse_label(split[1]){
                Ok(Label) => Ok(PPush(parse_label(split[1])?)),
                Err(e) => Ok(PI(Instr::from_str(s)?))
            },
            _ => match parse_label(split[0]){
                Ok(Label) => Ok(PLabel(parse_label(split[0])?)),
                Err(e) => Ok(PI(Instr::from_str(s)?))
            }
        }
    }
}

/// Test to_string and from_string implementations (to_string comes
/// for free from Display).
#[test]
fn test_isa_parse() -> Result<(), ParseError> {
    assert_eq!(PLabel("Ltest".into()), PLabel("Ltest".into()).to_string().parse()?);
    assert_eq!(PPush("Ltest".into()), PPush("Ltest".into()).to_string().parse()?);
    let pinstrs: Vec<PInstr> = vec![Push(Vi32(123)), Pop, Peek(45), Unary(Neg),
				    Binary(Lt), Swap, Alloc, Set, Get, Var(65),
				    Store(5), Call, Ret, Branch, Halt]
	.into_iter().map(|x| PI(x)).collect();
    for pinstr in pinstrs {
	assert_eq!(pinstr, pinstr.to_string().parse()?);
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////
// ToBytes trait implementations
////////////////////////////////////////////////////////////////////////

impl ToBytes for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        return self.to_be_bytes().to_vec();
    }
}

impl ToBytes for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        return self.to_be_bytes().to_vec();
    }
}

impl ToBytes for Unop {
    fn to_bytes(&self) -> Vec<u8> {
        return vec![0x00]
    }
}

impl ToBytes for Binop {
    fn to_bytes(&self) -> Vec<u8> {
        match self{
            Add => return vec![0x00],
            Mul => return vec![0x01],
            Sub => return vec![0x02],
            Div => return vec![0x03],
            Lt => return vec![0x04],
            Eq => return vec![0x05]
        }
    }
}

impl ToBytes for Val {
    fn to_bytes(&self) -> Vec<u8> {
        match self{
            Vunit => return vec![0x00],
            Vi32(i) => return [vec![0x01], i32::to_bytes(i)].concat(),
            Vbool(b) => {
                if *b == true{
                    return vec![0x02]
                }
                else{
                    return vec![0x03]
                }
            },
            Vloc(u) => return [vec![0x04], u32::to_bytes(u)].concat(),
            Vundef => return vec![0x05],
            Vsize(i32) => return vec![0x11],
            Vaddr(Address) => return vec![0x11],
        }
    }
}

impl ToBytes for Instr {
    fn to_bytes(&self) -> Vec<u8> {
        match self{
            Push(v) => return [vec![0x00], Val::to_bytes(v)].concat(),
            Pop => return vec![0x01],
            Peek(v) => return [vec![0x02], u32::to_bytes(v)].concat(),
            Unary(v) => return [vec![0x03], Unop::to_bytes(v)].concat(),
            Binary(b) => return [vec![0x04], Binop::to_bytes(b)].concat(),
            Swap => return vec![0x05],
            Alloc => return vec![0x06],
            Set => return vec![0x07],
            Get => return vec![0x08],
            Var(v) => return [vec![0x09], u32::to_bytes(v)].concat(),
            Store(v) => return [vec![0x0A], u32::to_bytes(v)].concat(),
            SetFrame(v) => return [vec![0x0B], u32::to_bytes(v)].concat(),
            Call => return vec![0x0C],
            Ret => return vec![0x0D],
            Branch => return vec![0x0E],
            Halt => return vec![0x0F],
        }
    }
}

// Put all your test cases in this module.
#[cfg(test)]
mod tests {
    use super::*;

    // Example test case.
    #[test]
    fn test_1() {
        assert_eq!(Instr::from_str("push 123").unwrap(), Push(Vi32(123)));
        assert_eq!(PInstr::from_str("Labc123:").unwrap(),
		   PLabel(String::from("Labc123"))
        );
    }
    #[test]
    fn unit_test1(){
        assert_eq!(Instr::from_str("push 12").unwrap(), Push(Vi32(12)));
        assert_eq!(Binop::from_str("+").unwrap(), Binop::from(Add));
    }
    #[test]
    fn unit_test2(){
        assert_eq!(Binop::to_bytes(&Add), vec![0x00]);
        assert_eq!(Val::to_bytes(&Vi32(1)), vec![1,0,0,0,1]);
    }
    #[test]
    fn unit_test3(){
        assert_eq!(Instr::from_str("push 700").unwrap(), Push(Vi32(700)));
        assert_eq!(Val::to_bytes(&Vi32(700)), vec![1,0,0,2,188]);
    }
}
