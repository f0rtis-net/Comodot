use std::fmt;
use std::fmt::Formatter;

pub mod keywords;
pub mod reserved_symbols;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Token {
    INTEGER(i64),
    REAL(f64),
    PLUS,
    MINUS,
    SLASH,
    STAR,
    LBRACKET,
    RBRACKET
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}