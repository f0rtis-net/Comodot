use std::iter::Peekable;
use std::str::Chars;

pub struct Cursor<'a> {
    chars: Peekable<Chars<'a>>,
    column: usize,
    line: usize,
}

const EOF: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(text_source: &'a str) -> Cursor<'a> {
        Self {
            chars: text_source.chars().peekable(),
            column: 1,
            line: 1,
        }
    }

    pub fn peek(&mut self) -> &char {
        self.chars.peek().unwrap_or(&EOF)
    }

    pub fn first(&mut self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    pub fn next(&mut self) -> char {
        let mut iter = self.chars.clone();
        iter.next();

        iter.next().unwrap_or(EOF)
    }

    pub fn bump(&mut self) -> Option<char> {
        let symbol = self.chars.next()?;

        self.update_position(symbol);

        Some(symbol)
    }

    pub fn skip_until(&mut self, predicate: fn(&char) -> bool) {
        while predicate(&self.first()) && self.first() != EOF {
            let symbol = self.first();
            self.update_position(symbol);
            self.bump();
        }
    }

    fn update_position(&mut self, symbol: char) {
        if symbol == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }

    pub fn column(&self) -> usize {self.column}

    pub fn line(&self) -> usize {self.line}
}