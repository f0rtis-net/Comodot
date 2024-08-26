use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::Token;

lazy_static!(
    pub static ref RESERVED_SYMBOLS: HashMap<char, Token> = HashMap::from([
        ('+', Token::PLUS),
        ('-', Token::MINUS),
        ('/', Token::SLASH),
        ('*', Token::STAR),
        ('(', Token::LBRACKET),
        (')', Token::RBRACKET),
    ]);
);