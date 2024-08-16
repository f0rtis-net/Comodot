use std::ops::Deref;
use crate::tokens::{NumberBase, Position, Token, TokenType};
use crate::keywords::KEYWORDS;
use crate::reserved_symbols::RESERVED_SYMBOLS;
pub mod tokens;
pub mod cursor;
mod keywords;
mod tests;
mod reserved_symbols;

/*
РАЗНЕСИ КОД В 2 РАЗНЫХ РЕПОЗИТОРИЯ
ПЕРВЫЙ ЭТО ФРОНТЕД ЯЗЫКА
ВТОРОЙ ЭТО БЭКЕНД ЯЗЫКА
 */

pub struct Lexer<'a> {
    cursor: cursor::Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            cursor: cursor::Cursor::new(source),
        }
    }

    fn try_to_parse_identifier(&mut self, symbol: char) -> Option<String> {
        if !symbol.is_ascii_alphanumeric() {
            return None;
        }

        let mut identifier = String::from(symbol);

        while self.cursor.peek().is_some() && self.cursor.peek()?.is_ascii_alphanumeric() {
            let val = self.cursor.next()?;

            identifier.push(val);
        }

        Some(identifier)
    }

    fn try_to_parse_number(&mut self, symbol: char) -> Option<(TokenType, String)> {
        let mut number = String::from(symbol);

        let base = if symbol == '0' {

            match self.cursor.next()? {
                'b' => NumberBase::BINARY,

                'x' => NumberBase::HEX,

                'o' => NumberBase::OCTAL,

                '0'..='9' => NumberBase::DECIMAL,

                _ => return Some(
                    (TokenType::NUMBER(NumberBase::DECIMAL), number)
                )
            }
        } else {
            NumberBase::DECIMAL
        };

        while self.cursor.peek().is_some() {
            let val = self.cursor.next()?;

            if val == '_' {
                continue
            }

            if !val.is_ascii_alphanumeric() {
                break
            }

            number.push(val);
        }

        Some((TokenType::NUMBER(base), number))
    }

    fn try_to_parse_double_char(&mut self, second: char, result: &str, single_type: TokenType, double_type: TokenType) -> (TokenType, String) {
        if self.cursor.peek().unwrap_or('\0') == second {
            self.cursor.next();
            (double_type, String::from(result))
        } else {
            (single_type, String::from(result))
        }
    }

    pub fn process_token(&mut self) -> Result<Option<Token>, String> {
        let first = match self.cursor.next() {

            Some(c) => c,

            None => return Ok(None),
        };

        let mut token_type = TokenType::UNDEFINED;
        let mut token_value = String::new();

        if first.is_whitespace() {
            self.cursor.skip_until( |ch| ch.is_whitespace() );
            return self.process_token()
        }

        (token_type, token_value) = match RESERVED_SYMBOLS.get(&first) {

            Some(type_) => (*type_.deref(), String::from(first)),

            _ => {
                match first {

                    '0'..='9' => match self.try_to_parse_number(first) {
                        Some(num) => num,
                        None => return Ok(None)
                    }

                    '&' => self.try_to_parse_double_char(
                        '&',
                        "&&",
                        TokenType::AMPERSAND,
                        TokenType::AND
                    ),

                    '=' => self.try_to_parse_double_char(
                        '=',
                        "==",
                        TokenType::EQ,
                        TokenType::DoubleEQ
                    ),

                    '-' => self.try_to_parse_double_char(
                        '>',
                        "->",
                        TokenType::MINUS,
                        TokenType::ARROW
                    ),

                    '|' => self.try_to_parse_double_char(
                        '|',
                        "||",
                        TokenType::VerticalSlash,
                        TokenType::OR
                    ),

                    _ => {
                        match self.try_to_parse_identifier(first) {

                            Some(keyword) => {
                                if KEYWORDS.contains(&keyword.as_str()) {
                                    (TokenType::KEYWORD, String::from(keyword))
                                } else {
                                    (TokenType::IDENTIFIER, String::from(keyword))
                                }
                            }

                            None => return Err (
                                format!("Can't process char: {} at line {} | column {}", first, self.cursor.line(), self.cursor.column())
                            ),
                        }
                    }
                }
            }
        };

        if token_type == TokenType::UNDEFINED {
            return Err (
                format!("Can't process token with start char: {} at line {} | column {}", first, self.cursor.line(), self.cursor.column())
            )
        }

        Ok(Some(Token {
            type_: token_type,
            value: token_value,
            position: Position {
                line: self.cursor.line(),
                column: self.cursor.column(),
            },
        }))
    }
}