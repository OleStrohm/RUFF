mod reg;

use reg::Register;

use std::fmt::Formatter;
use std::fs;
use std::{error::Error, fmt::Display};

type ParseResult<'a, T> = Result<T, ParseError<'a>>;

#[derive(Debug)]
enum Instruction {
    Label(String),
    Mov(Register, u32),
}

#[derive(Debug)]
enum ParseError<'a> {
    ExpectedEOL(&'a str),
    NotAReg(&'a str),
    ExpectedIdent,
    ExpectedNumber,
    ExpectedColon,
    ExpectedComma,
    ExpectedReg,
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ParseError::ExpectedEOL(junk) => write!(f, "Expected EOL, found `{}`", junk),
            ParseError::NotAReg(bad_reg) => write!(f, "`{}` is not a register", bad_reg),
            ParseError::ExpectedIdent => write!(f, "Expected label or op"),
            ParseError::ExpectedNumber => write!(f, "Expected number"),
            ParseError::ExpectedColon => write!(f, "Expected `:` after label"),
            ParseError::ExpectedComma => write!(f, "Expected `,` between parameters"),
            ParseError::ExpectedReg => write!(f, "Expected register"),
        }
    }
}

impl Error for ParseError<'_> {
    fn description(&self) -> &str {
        match self {
            ParseError::ExpectedEOL(_) => "Found junk at end of line",
            ParseError::NotAReg(_) => "Register does not exist",
            ParseError::ExpectedIdent => "Expected label or op",
            ParseError::ExpectedNumber => "Expected number",
            ParseError::ExpectedColon => "Expected `:` after label",
            ParseError::ExpectedComma => "Expected `,` between parameters",
            ParseError::ExpectedReg => "Expected register",
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SrcLine<'a>(&'a str);

impl<'a> SrcLine<'a> {
    fn discard_whitespace(&mut self) {
        self.0 = self.0.trim_start();
    }

    fn consume_ident(&mut self) -> ParseResult<'a, &'a str> {
        self.discard_whitespace();
        if self.peak() != Peeked::Ident {
            return Err(ParseError::ExpectedIdent);
        }

        let is_valid_ident = |c: &char| c.is_alphabetic() || c.is_digit(10);
        let end_byte = self
            .0
            .chars()
            .take_while(is_valid_ident)
            .map(char::len_utf8)
            .sum();
        let (ident, rest) = self.0.split_at(end_byte);
        self.0 = rest;
        Ok(ident)
    }

    fn consume_reg(&mut self) -> ParseResult<'a, Register> {
        self.discard_whitespace();
        if self.peak() != Peeked::Ident {
            return Err(ParseError::ExpectedReg);
        }

        let reg = self.consume_ident().unwrap();
        if reg.chars().next() == Some('R') {
            match reg[1..].parse::<u32>() {
                Ok(v) if v < 16 => Ok(v.into()),
                _ => Err(ParseError::NotAReg(reg)),
            }
        } else {
            Err(ParseError::ExpectedReg)
        }
    }

    fn expect_colon(&mut self) -> ParseResult<'a, ()> {
        self.discard_whitespace();
        if self.peak() != Peeked::Colon {
            return Err(ParseError::ExpectedColon);
        }
        self.skip(1);
        Ok(())
    }

    fn expect_comma(&mut self) -> ParseResult<'a, ()> {
        self.discard_whitespace();
        if self.peak() != Peeked::Comma {
            return Err(ParseError::ExpectedComma);
        }
        self.skip(1);
        Ok(())
    }

    fn consume_num(&mut self) -> ParseResult<'a, u32> {
        self.discard_whitespace();
        if self.peak() != Peeked::Numeric {
            return Err(ParseError::ExpectedNumber);
        }

        let is_valid_ident = |c: &char| c.is_digit(10);
        let end_byte = self
            .0
            .chars()
            .take_while(is_valid_ident)
            .map(char::len_utf8)
            .sum();

        let (ident, rest) = self.0.split_at(end_byte);
        self.0 = rest;
        Ok(ident.parse::<u32>().unwrap())
    }

    fn peak(&self) -> Peeked {
        match self.0.trim_start().chars().next() {
            Some(c) => {
                if c.is_alphabetic() {
                    Peeked::Ident
                } else if c.is_digit(10) {
                    Peeked::Numeric
                } else if c == ':' {
                    Peeked::Colon
                } else if c == ',' {
                    Peeked::Comma
                } else {
                    Peeked::Invalid
                }
            }
            None => Peeked::EOL,
        }
    }

    fn skip(&mut self, s: usize) -> &'a str {
        self.0 = &self.0[s..];
        self.0
    }

    fn parse(mut self) -> ParseResult<'a, Instruction> {
        let ident = self.consume_ident()?;

        match ident {
            "mov" => {
                let reg = self.consume_reg()?;
                self.expect_comma()?;
                let immediate = self.consume_num()?;
                Ok(Instruction::Mov(reg, immediate))
            }
            _ => {
                self.expect_colon()?;
                match self.peak() {
                    Peeked::EOL => Ok(Instruction::Label(ident.into())),
                    _ => Err(ParseError::ExpectedEOL(self.skip(1))),
                }
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("assets/test.rasm").expect("Could not read file");

    contents
        .lines()
        .map(SrcLine)
        .for_each(|line| println!("{:?}", line.parse()));
}

#[derive(Debug, PartialEq)]
enum Peeked {
    Ident,
    Numeric,
    Colon,
    Comma,
    Invalid,
    EOL,
}
