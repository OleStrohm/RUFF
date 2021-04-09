mod error;
mod instr;
mod reg;

use codespan_reporting::{
    files::SimpleFiles,
    term::{
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use error::*;
use instr::Instruction;
use reg::Register;
use std::fs;

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

    fn expect(&mut self, p: Peeked) -> ParseResult<()> {
        self.discard_whitespace();
        if self.peak() != p {
            return Err(ParseError::Expected(p, self.loc));
        }
        if p != Peeked::EOL {
            self.line = &self.line[1..];
            self.loc += 1;
        }
        Ok(())
    }

    fn expect_colon(&mut self) -> ParseResult<()> {
        self.expect(Peeked::Colon)
    }

    fn expect_comma(&mut self) -> ParseResult<()> {
        self.expect(Peeked::Comma)
    }

    fn expect_eol(&mut self) -> ParseResult<()> {
        self.expect(Peeked::EOL)
    }

    fn consume_while<F: Fn(&char) -> bool>(&mut self, f: F) -> Result<&'a str, ()> {
        self.discard_whitespace();
        let end_byte = self.line.chars().take_while(f).map(char::len_utf8).sum();
        if end_byte == 0 {
            Err(())
        } else {
            let (ident, rest) = self.line.split_at(end_byte);
            self.loc += end_byte;
            self.line = rest;
            Ok(ident)
        }
    }

    fn consume_ident(&mut self) -> ParseResult<&'a str> {
        let is_valid_ident = |c: &char| c.is_alphabetic() || c.is_digit(10);

        self.consume_while(is_valid_ident)
            .map_err(|()| ParseError::Expected(Peeked::Ident, self.loc))
    }

    fn consume_num(&mut self) -> ParseResult<u32> {
        let is_valid_num = |c: &char| c.is_digit(10);
        self.consume_while(is_valid_num)
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .map_err(|()| ParseError::Expected(Peeked::Numeric, self.loc))
    }

    fn consume_reg(&mut self) -> ParseResult<Register> {
        self.discard_whitespace();
        let start_loc = self.loc;
        let reg = self
            .consume_ident()
            .map_err(|_| ParseError::ExpectedReg(self.loc))?;

        if reg.chars().next() == Some('R') {
            match reg[1..].parse::<u32>() {
                Ok(v) if v < 16 => Ok(v.into()),
                _ => Err(ParseError::NotAReg(start_loc..self.loc)),
            }
        } else {
            Err(ParseError::NotAReg(start_loc..self.loc))
        }
    }

    fn peak(&mut self) -> Peeked {
        self.discard_whitespace();
        match self.line.chars().next() {
            Some(c) if c.is_digit(10) => Peeked::Numeric,
            Some(c) if c == ':' => Peeked::Colon,
            Some(c) if c == ',' => Peeked::Comma,
            Some(c) if c.is_alphabetic() => Peeked::Ident,
            Some(_) => Peeked::Ident,
            None => Peeked::EOL,
        }
    }

    fn parse(mut self) -> ParseResult<Instruction> {
        let ident = self.consume_ident()?;

        match ident {
            "mov" => {
                let reg = self.consume_reg()?;
                self.expect_comma()?;
                let immediate = self.consume_num()?;
                self.expect_eol()?;
                Ok(Instruction::Mov(reg, immediate))
            }
            _ => {
                self.expect_colon()?;
                self.expect_eol()?;
                Ok(Instruction::Label(ident.into()))
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
                let diagnostic = err.diagnostic(offset, line.line, file_id);
                codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &diagnostic)
                    .unwrap();
            }
        };
        offset += line.line.len() + 1;
    });
}

#[derive(Debug, PartialEq, Eq)]
enum Peeked {
    Ident,
    Numeric,
    Colon,
    Comma,
    EOL,
}
