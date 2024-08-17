use std::iter::Peekable;
use std::str::Chars;

pub struct Cursor<'a> {
    text_source: Peekable<Chars<'a>>,
    column: i32,
    line: i32,
}

impl<'a> Cursor<'a> {
    pub fn new(text_source: &'a str) -> Cursor<'a> {
        Self {
            text_source: text_source.chars().peekable(),
            column: 1,
            line: 1,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        let symbol = *self.text_source.peek()?;

        Some(symbol)
    }

    pub fn next(&mut self) -> Option<char> {
        let symbol = self.text_source.next()?;

        self.update_position(symbol);

        Some(symbol)
    }

    pub fn skip_until(&mut self, predicate: fn(&char) -> bool) {
        while self.peek().is_some() && predicate(&self.peek().unwrap()) {
            let symbol = self.text_source.next().unwrap();

            self.update_position(symbol);

            self.next();
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

    pub fn column(&self) -> i32 {self.column}

    pub fn line(&self) -> i32 {self.line}
}