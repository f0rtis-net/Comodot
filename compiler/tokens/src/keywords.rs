use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::Token;

lazy_static! {
    pub static ref RESERVED_KEYWORDS: HashMap<&'static str, Token<'static>>  = HashMap::from([
        ("fn", Token::FUNCTION),
        ("ret", Token::RETURN),
        ("const", Token::CONST),
        ("pub", Token::PUBLIC),
        ("import", Token::IMPORT),
        ("true", Token::BOOL(true)),
        ("false", Token::BOOL(false)),
        ("if", Token::IF),
        ("else", Token::ELSE),
        ("extern", Token::EXTERN),
        ("val", Token::VAL),
    ]);
}