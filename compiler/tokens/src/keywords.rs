use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::Token;

lazy_static! {
    pub static ref RESERVED_KEYWORDS: HashMap<&'static str, Token>  = HashMap::from([
        ("func", Token::FUNCTION),
        ("ret", Token::RETURN),
        ("const", Token::CONST),
        ("pub", Token::PUBLIC),
    ]);
}