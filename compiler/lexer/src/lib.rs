use std::ops::{Add, Deref};
use tokens::reserved_symbols::RESERVED_SYMBOLS;
use tokens::Token;
use tokens::Token::{INTEGER, REAL};
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
    fn process_number(&mut self, first: char) -> LexerResult<Token, usize, &'static str> {
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
        let mut result_number = String::from(first);

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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<Token, usize, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.cursor.bump()?;

        if first.is_whitespace() {
            self.cursor.skip_until( |ch| ch.is_whitespace() );
            return self.next()
        }

        if let Some(token) = RESERVED_SYMBOLS.get(&first) {
            return Some(
                Ok((
                    self.cursor.column(),
                    *token,
                    self.cursor.line(),
                ))
            );
        }

        match first {
            c @ '0'..='9' => Some(self.process_number(c)),
            _ => Some(Err("Unknown symbol"))
        }
    }
}