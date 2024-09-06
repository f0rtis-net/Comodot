use std::ops::{Add, Deref};
use tokens::keywords::RESERVED_KEYWORDS;
use tokens::Token;
use tokens::Token::{IDENTIFIER, INTEGER, REAL};
use crate::cursor::Cursor;
use crate::DigitBase::DECIMAL;

pub mod cursor;
mod tests;

pub type LexerResult<Tok, Loc, Err> = Result<(Loc, Tok, Loc), Err>;

#[derive(Eq, PartialEq)]
enum DigitBase {
    HEX,
    OCTAL,
    DECIMAL,
    BINARY,
}

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {

    pub fn new(input: &'a str) -> Lexer<'a> {
        Self {
            cursor: Cursor::new(input),
        }
    }
    fn process_number(&mut self, first: char, neg: bool) -> LexerResult<Token, usize, &'static str> {
        // select number base
        let mut base = DECIMAL;

        let mut real_flag = false;

        if first == '0' {
            base = match self.cursor.first() {
                'x' => DigitBase::HEX,
                'b' => DigitBase::BINARY,
                'o' => DigitBase::OCTAL,
                '0'..='9' | '_' => DECIMAL,
                '.' => {real_flag = true; DECIMAL},

                //just a zero
                _ => return Ok((self.cursor.column(), INTEGER(0), self.cursor.line()))
            };

            //scip base marker symbols
            if base != DECIMAL || (base == DECIMAL && real_flag) {
                self.cursor.bump();
            }
        }

        // process number
        let mut result_number = String::new();

        if neg {
            result_number.push('-');
        }

        result_number.push(first);

        loop {
            match self.cursor.first() {
                '_' => { self.cursor.bump(); },
                '0'..='9' => result_number.push(self.cursor.bump().unwrap()),
                'a'..='f' | 'A'..='F' => {
                    if base == DigitBase::HEX  {
                        result_number.push(self.cursor.bump().unwrap())
                    } else {
                        return Err("Invalid digits in non hexadecimal number");
                    }
                },
                _ => break
            }
        }

        if real_flag {
            if base != DECIMAL {
                return Err("Invalid number format parsed");
            }

            return Ok((
                self.cursor.column(),
                REAL(result_number.parse::<f64>().unwrap()),
                self.cursor.line()
            ));
        }

        //convert to integer

        let result = match base {
            DECIMAL => i64::from_str_radix(result_number.as_str(), 10),
            DigitBase::HEX => i64::from_str_radix(result_number.as_str(), 16),
            DigitBase::OCTAL => i64::from_str_radix(result_number.as_str(), 8),
            DigitBase::BINARY => i64::from_str_radix(result_number.as_str(), 2),
        };

        if result.is_err() {
            return Err("Invalid number format parsed");
        }

        Ok((
            self.cursor.column(),
            INTEGER(result.unwrap()),
            self.cursor.line()
        ))
    }

    fn process_id(&mut self, first: char) -> LexerResult<Token, usize, &'static str> {
        let mut result = String::from(first);

        loop {
            match self.cursor.first() {
                c if c.is_ascii_alphanumeric() => { result.push(self.cursor.bump().unwrap()) },
                _ => break
            };
        }

        if let Some(token) = RESERVED_KEYWORDS.get(result.as_str()) {
            return Ok((
                self.cursor.column(),
                *token,
                self.cursor.line(),
            ));
        }

        Ok((
            self.cursor.column(),
            IDENTIFIER(result.leak()),
            self.cursor.line()
        ))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<Token, usize, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.cursor.bump()?;

        if first.is_whitespace() {
            self.cursor.skip_until( |ch| ch.is_whitespace() );
            return self.next()
        }

        let tok_type = match first {
            '+' => Token::PLUS,
            '/' => Token::SLASH,
            '*' => Token::STAR,
            '(' => Token::LBRACKET,
            ')' => Token::RBRACKET,
            '{' => Token::LRBRACKET,
            '}' => Token::RRBRACKET,
            ';' => Token::SEMICOLON,
            '>' => Token::GT,
            '<' => Token::LT,
            '=' => Token::EQ,

            '-' => {
                match self.cursor.first() {
                    c @ '0'..='9' => {self.cursor.bump(); return Some(self.process_number(c, true))},
                    _ => Token::MINUS,
                }
            }

            c if c.is_ascii_alphabetic() => return Some(self.process_id(c)),
            c @ '0'..='9' => return Some(self.process_number(c, false)),
            _ => return Some(Err("Unknown symbol"))
        };

        Some(Ok((self.cursor.column(), tok_type, self.cursor.line())))
    }
}