use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::TokenType;

lazy_static!(
    pub static ref RESERVED_SYMBOLS: HashMap<char, TokenType> = HashMap::from([
        ('#', TokenType::GRID),
        ('!', TokenType::EXCLAMATION),
        ('*', TokenType::STAR),
        ('(', TokenType::LBRACE),
        (')', TokenType::RBRACE),
        ('{', TokenType::LFBRACE),
        ('}', TokenType::RFBRACE),
        ('[', TokenType::LStraitBrace),
        (']', TokenType::RStraitBrace),
        ('+', TokenType::PLUS),
        ('/', TokenType::SLASH),
        ('%', TokenType::PERCENT),
        ('"', TokenType::DoubleQuote),
        ('\'', TokenType::QUOTE),
        ('\\', TokenType::BackSlash),
        (';', TokenType::DotComma),
        (':', TokenType::DoubleDot),
        (',', TokenType::COMMA),
        ('<', TokenType::LT),
        ('>', TokenType::GT),
        ('^', TokenType::UpArrow),
        ('~', TokenType::UpArrow),
    ]);
);