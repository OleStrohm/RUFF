use crate::lexer::Token;
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::diagnostic::Label;
use std::convert::Into;
use std::ops::Range;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeSpan(pub usize, pub usize);

impl From<CodeSpan> for Range<usize> {
    fn from(CodeSpan(start, end): CodeSpan) -> Self {
        start..end
    }
}

#[derive(Debug)]
pub enum ParseError {
    NotAReg(CodeSpan),
    ExpectedIdent(Token),
    ExpectedReg(Token),
    ExpectedNumber(Token),
    ExpectedColon(Token),
    ExpectedComma(Token),
    ExpectedEOL(Token),
    UnexpectedEOL,
}

impl ParseError {
    pub fn diagnostic<FileId>(
        &self,
        offset: usize,
        line: &str,
        file_id: FileId,
    ) -> Diagnostic<FileId> {
        let span: Range<_> = match *self {
            ParseError::NotAReg(span) => span,
            ParseError::ExpectedReg(token) => token.into(),
            ParseError::ExpectedIdent(token) => token.into(),
            ParseError::ExpectedNumber(token) => token.into(),
            ParseError::ExpectedColon(token) => token.into(),
            ParseError::ExpectedComma(token) => token.into(),
            ParseError::ExpectedEOL(token) => CodeSpan(CodeSpan::from(token).0, line.len()),
            ParseError::UnexpectedEOL => CodeSpan(line.len()-1, line.len()-1),
        }
        .into();

        let label = match self {
            ParseError::NotAReg(_) => {
                format!("`{}` is not a valid register", &line[span.clone()])
            }
            ParseError::ExpectedReg(_) => format!("Not a register"),
            ParseError::ExpectedEOL(_) => {
                format!("Expected EOL, found `{}`", &line[span.clone()])
            }
            ParseError::ExpectedIdent(_) => "Expected label or op".into(),
            ParseError::ExpectedNumber(_) => "Expected number".into(),
            ParseError::ExpectedColon(_) => "Expected `:` here".into(),
            ParseError::ExpectedComma(_) => "Expected `,` here".into(),
            ParseError::UnexpectedEOL => "Unexpected EOL".into(),
        };

        let span = Range {
            start: span.start + offset,
            end: span.end + offset,
        };

        Diagnostic::error()
            .with_message(self.description())
            .with_labels(vec![Label::primary(file_id, span).with_message(label)])
    }

    pub fn description(&self) -> &str {
        match self {
            ParseError::NotAReg(_) => "Register does not exist",
            ParseError::ExpectedReg(_) => "Expected a register",
            ParseError::ExpectedIdent(_) => "Expected label or op",
            ParseError::ExpectedNumber(_) => "Expected number",
            ParseError::ExpectedColon(_) => "Expected `:` after label",
            ParseError::ExpectedComma(_) => "Expected `,` between parameters",
            ParseError::ExpectedEOL(_) => "Found junk at end of line",
            ParseError::UnexpectedEOL => "Line ended prematurely",
        }
    }
}
