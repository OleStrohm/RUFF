use std::fmt::Display;
use std::fmt::Formatter;

use crate::error::*;
use crate::lexer::{Lexer, Token};
use crate::reg::Register;

#[derive(Debug)]
pub enum ParsedInstruction {
    Label(String),
    Mov(Register, u32),
    Jmp(String),
}

#[derive(Debug, Clone, Copy)]
pub struct ParseLine<'a> {
    lexer: Lexer<'a>,
}

impl<'a> ParseLine<'a> {
    pub fn new(line: &'a str) -> Self {
        Self {
            lexer: Lexer::new(line),
        }
    }

    fn read_span(&self, span: CodeSpan) -> &'a str {
        self.lexer.line(span)
    }

    fn expect_colon(&mut self) -> ParseResult<()> {
        match self.lexer.next() {
            Some(Token::Colon(_)) => Ok(()),
            Some(t) => Err(ParseError::ExpectedColon(t)),
            None => Err(ParseError::UnexpectedEOL),
        }
    }

    fn expect_comma(&mut self) -> ParseResult<()> {
        match self.lexer.next() {
            Some(Token::Comma(_)) => Ok(()),
            Some(t) => Err(ParseError::ExpectedComma(t)),
            None => Err(ParseError::UnexpectedEOL),
        }
    }

    fn expect_eol(&mut self) -> ParseResult<()> {
        match self.lexer.next() {
            None => Ok(()),
            Some(t) => Err(ParseError::ExpectedEOL(t)),
        }
    }

    fn consume_ident(&mut self) -> ParseResult<&'a str> {
        match self.lexer.next() {
            Some(Token::Ident(span)) => Ok(self.read_span(span)),
            Some(t) => Err(ParseError::ExpectedIdent(t)),
            None => Err(ParseError::UnexpectedEOL),
        }
    }

    fn consume_num(&mut self) -> ParseResult<u32> {
        match self.lexer.next() {
            Some(Token::Number(span)) => Ok(self.read_span(span).parse::<u32>().unwrap()),
            Some(t) => Err(ParseError::ExpectedNumber(t)),
            None => Err(ParseError::UnexpectedEOL),
        }
    }

    fn consume_reg(&mut self) -> ParseResult<Register> {
        let (reg, span) = match self.lexer.next() {
            Some(Token::Ident(span)) => Ok((self.read_span(span), span)),
            Some(t) => Err(ParseError::ExpectedReg(t)),
            None => Err(ParseError::UnexpectedEOL),
        }?;

        if reg.chars().next() == Some('R') {
            match reg[1..].parse::<u32>() {
                Ok(v) if v < 16 => Ok(v.into()),
                _ => Err(ParseError::NotAReg(span)),
            }
        } else {
            Err(ParseError::NotAReg(span))
        }
    }

    pub fn parse(mut self) -> ParseResult<ParsedInstruction> {
        let ident = self.consume_ident()?;

        Ok(match ident {
            "mov" => {
                let reg = self.consume_reg()?;
                self.expect_comma()?;
                let immediate = self.consume_num()?;
                self.expect_eol()?;
                ParsedInstruction::Mov(reg, immediate)
            }
            "jmp" => {
                let label = self.consume_ident()?;
                self.expect_eol()?;
                ParsedInstruction::Jmp(label.into())
            }
            _ => {
                self.expect_colon()?;
                self.expect_eol()?;
                ParsedInstruction::Label(ident.into())
            }
        })
    }
}

impl Display for ParsedInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let line = match self {
            ParsedInstruction::Label(l) => format!("{}:", l),
            ParsedInstruction::Mov(reg, imm) => format!("    mov {:?}, {}", reg, imm),
            ParsedInstruction::Jmp(l) => format!("    jmp {}", l),
        };

        f.pad(&line)
    }
}
