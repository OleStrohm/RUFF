mod reg;

use codespan_reporting::diagnostic::Label;
use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFiles,
    term::{
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use reg::Register;
use std::ops::Range;

use std::fs;

type ParseResult<'a, T> = Result<T, ParseError<'a>>;

#[derive(Debug)]
enum Instruction {
    Label(String),
    Mov(Register, u32),
}

#[derive(Debug)]
enum ParseError<'a> {
    ExpectedEOL(&'a str, Range<usize>),
    NotAReg(&'a str, Range<usize>),
    ExpectedIdent(usize),
    ExpectedNumber(usize),
    ExpectedColon(usize),
    ExpectedComma(usize),
    ExpectedReg(usize),
}

impl ParseError<'_> {
    fn diagnostic<FileId>(&self, offset: usize, file_id: FileId) -> Diagnostic<FileId> {
        let label = match self {
            ParseError::ExpectedEOL(junk, _) => format!("Expected EOL, found `{}`", junk),
            ParseError::NotAReg(bad_reg, _) => format!("`{}` is not a register", bad_reg),
            ParseError::ExpectedIdent(_) => String::from("Expected label or op"),
            ParseError::ExpectedNumber(_) => String::from("Expected number"),
            ParseError::ExpectedColon(_) => String::from("Expected `:` after label"),
            ParseError::ExpectedComma(_) => String::from("Expected `,` between parameters"),
            ParseError::ExpectedReg(_) => String::from("Expected register"),
        };

        let span = match self {
            ParseError::ExpectedEOL(_, span) => span.clone(),
            ParseError::NotAReg(_, span) => span.clone(),
            ParseError::ExpectedIdent(loc) => *loc..(*loc + 1),
            ParseError::ExpectedNumber(loc) => *loc..(*loc + 1),
            ParseError::ExpectedColon(loc) => *loc..(*loc + 1),
            ParseError::ExpectedComma(loc) => *loc..(*loc + 1),
            ParseError::ExpectedReg(loc) => *loc..(*loc + 1),
        };

        let span = Range {
            start: span.start + offset,
            end: span.end + offset,
        };

        Diagnostic::error()
            .with_message(self.description())
            .with_labels(vec![Label::primary(file_id, span).with_message(label)])
    }
}

impl ParseError<'_> {
    fn description(&self) -> &str {
        match self {
            ParseError::ExpectedEOL(_, _) => "Found junk at end of line",
            ParseError::NotAReg(_, _) => "Register does not exist",
            ParseError::ExpectedIdent(_) => "Expected label or op",
            ParseError::ExpectedNumber(_) => "Expected number",
            ParseError::ExpectedColon(_) => "Expected `:` after label",
            ParseError::ExpectedComma(_) => "Expected `,` between parameters",
            ParseError::ExpectedReg(_) => "Expected register",
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SrcLine<'a> {
    line: &'a str,
    loc: usize,
}

impl<'a> SrcLine<'a> {
    fn new(line: &'a str) -> Self {
        Self { line, loc: 0 }
    }

    fn discard_whitespace(&mut self) {
        let new = self.line.trim_start();
        self.loc += self.line.len() - new.len();
        self.line = new;
    }

    fn consume_ident(&mut self) -> ParseResult<'a, &'a str> {
        self.discard_whitespace();
        if self.peak() != Peeked::Ident {
            return Err(ParseError::ExpectedIdent(self.loc));
        }

        let is_valid_ident = |c: &char| c.is_alphabetic() || c.is_digit(10);
        let end_byte = self
            .line
            .chars()
            .take_while(is_valid_ident)
            .map(char::len_utf8)
            .sum();
        let (ident, rest) = self.line.split_at(end_byte);
        self.loc += end_byte;
        self.line = rest;
        Ok(ident)
    }

    fn consume_reg(&mut self) -> ParseResult<'a, Register> {
        self.discard_whitespace();
        if self.peak() != Peeked::Ident {
            return Err(ParseError::ExpectedReg(self.loc));
        }

        let start_loc = self.loc;
        let reg = self.consume_ident().unwrap();
        if reg.chars().next() == Some('R') {
            match reg[1..].parse::<u32>() {
                Ok(v) if v < 16 => Ok(v.into()),
                _ => Err(ParseError::NotAReg(reg, start_loc..self.loc)),
            }
        } else {
            Err(ParseError::NotAReg(reg, start_loc..self.loc))
        }
    }

    fn expect_colon(&mut self) -> ParseResult<'a, ()> {
        self.discard_whitespace();
        if self.peak() != Peeked::Colon {
            return Err(ParseError::ExpectedColon(self.loc));
        }
        self.skip(1);
        Ok(())
    }

    fn expect_comma(&mut self) -> ParseResult<'a, ()> {
        self.discard_whitespace();
        if self.peak() != Peeked::Comma {
            return Err(ParseError::ExpectedComma(self.loc));
        }
        self.skip(1);
        Ok(())
    }

    fn consume_num(&mut self) -> ParseResult<'a, u32> {
        self.discard_whitespace();
        if self.peak() != Peeked::Numeric {
            return Err(ParseError::ExpectedNumber(self.loc));
        }

        let is_valid_ident = |c: &char| c.is_digit(10);
        let end_byte = self
            .line
            .chars()
            .take_while(is_valid_ident)
            .map(char::len_utf8)
            .sum();

        let (ident, rest) = self.line.split_at(end_byte);
        self.line = rest;
        Ok(ident.parse::<u32>().unwrap())
    }

    fn peak(&mut self) -> Peeked {
        self.discard_whitespace();
        match self.line.chars().next() {
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

    fn skip(&mut self, s: usize) {
        self.line = &self.line[s..];
        self.loc += s;
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
                    _ => Err(ParseError::ExpectedEOL(
                        self.line,
                        self.loc..(self.loc + self.line.len()),
                    )),
                }
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("assets/test.rasm").expect("Could not read file");

    let mut files = SimpleFiles::new();

    let file_id = files.add("assets/test.rasm", contents.as_str());

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let mut offset = 0;

    contents.lines().map(SrcLine::new).for_each(|line| {
        match line.parse() {
            Ok(instr) => println!("{:?}", instr),
            Err(err) => {
                let diagnostic = err.diagnostic(offset, file_id);
                codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &diagnostic)
                    .unwrap();
            }
        };
        offset += line.line.len() + 1;
    });
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
