//! The Grumpy compiler.

#![warn(clippy::all)]
use std::{error, fmt, io, num};

// Declare 'isa' and 'assemble' as modules in the grumpy crate.
pub mod assemble;
pub mod isa;

// Trait for types that can be converted to a binary representation.
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

// A type for parse errors.
#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for ParseError {}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> Self {
        ParseError(format!("{}", err))
    }
}

impl From<ParseError> for io::Error {
    fn from(err: ParseError) -> Self {
        io::Error::new(io::ErrorKind::Other, format!("{:?}", err))
    }
}
