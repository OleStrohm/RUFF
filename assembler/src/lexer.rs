use std::ops::Range;
use crate::error::CodeSpan;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lexer<'a> {
    line: &'a str,
    loc: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(line: &'a str) -> Self {
        Self { line, loc: 0 }
    }

    pub fn line(&self, span: CodeSpan) -> &'a str {
        &self.line[Range::<_>::from(span)]
    }

    pub fn loc(&self) -> usize {
        self.loc
    }

    fn rest(&self) -> &'a str {
        if self.loc >= self.line.len() {
            ""
        } else {
            &self.line[self.loc..]
        }
    }

    fn consume_while<F>(&mut self, f: F) -> CodeSpan
    where
        F: Fn(&char) -> bool,
    {
        let end: usize = self.rest().chars().take_while(f).map(char::len_utf8).sum();
        let start = self.loc;
        self.loc += end;

        CodeSpan(start, start + end)
    }

    fn consume_n(&mut self, n: usize) -> CodeSpan {
        let end: usize = self.rest().chars().take(n).map(char::len_utf8).sum();
        let start = self.loc;
        self.loc += end;

        CodeSpan(start, start + end)
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let new = self.rest().trim_start();
        self.loc += self.rest().len() - new.len();

        Some(match self.rest().chars().next()? {
            c if c.is_digit(10) => {
                let is_numeric = |c: &char| c.is_digit(10);
                let span = self.consume_while(is_numeric);
                Token::Number(span)
            }
            ':' => Token::Colon(self.consume_n(1)),
            ',' => Token::Comma(self.consume_n(1)),
            c if c.is_alphabetic() => {
                let is_indent = |c: &char| c.is_digit(10) || c.is_alphabetic() || c == &'_';
                let span = self.consume_while(is_indent);
                Token::Ident(span)
            }
            _ => unimplemented!(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    Ident(CodeSpan),
    Number(CodeSpan),
    Colon(CodeSpan),
    Comma(CodeSpan),
}

impl From<Token> for CodeSpan {
    fn from(t: Token) -> Self {
        match t {
            Token::Ident(span) => span,
            Token::Number(span) => span,
            Token::Colon(span) => span,
            Token::Comma(span) => span,
        }
    }
}
