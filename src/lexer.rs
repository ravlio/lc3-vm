use crate::token::{TokenType, Token};

pub struct Lexer<'a> {
    input: Vec<char>,
    // start position of the current token
    start: usize,
    // current position
    pos: usize,
    // number of lines
    line: usize,

}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {}
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            start: 0,
            pos: 0,
            line: 0,
        }
    }
    fn next(&mut self) -> Option<char> {
        let c = match self.input.get(self.pos) {
            Some(c) => {
                self.pos += 1;
                if c == '\n' {
                    self.line += 1;
                }
                c
            }
            None => None,
        };
    }

    fn peek(&mut self) -> Option<char> {
        let c = self.next();
        self.backup();
        c
    }

    fn backup(&mut self) {
        self.pos -= 1;
        if self.input.get(self.pos) == '\n' {
            self.line -= 1;
        }
    }

    fn emit(&mut self, t: TokenType) {

    }
}