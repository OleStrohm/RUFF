use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::diagnostic::Label;
use std::ops::Range;

use crate::Peeked;

pub(super) type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub(super) enum ParseError {
    NotAReg(Range<usize>),
    ExpectedReg(usize),
    Expected(Peeked, usize),
}

impl ParseError {
    pub fn diagnostic<FileId>(
        &self,
        offset: usize,
        line: &str,
        file_id: FileId,
    ) -> Diagnostic<FileId> {
        let span = match self {
            ParseError::NotAReg(span) => span.clone(),
            ParseError::Expected(Peeked::EOL, loc) => *loc..line.len(),
            ParseError::ExpectedReg(loc) | ParseError::Expected(_, loc) => *loc..(*loc + 1),
        };

        let label = match self {
            ParseError::NotAReg(_) => {
                format!("`{}` is not a valid register", &line[span.clone()])
            }
            ParseError::ExpectedReg(_) => format!("Not a register"),
            ParseError::Expected(Peeked::EOL, _) => {
                format!("Expected EOL, found `{}`", &line[span.clone()])
            }
            ParseError::Expected(Peeked::Ident, _) => "Expected label or op".into(),
            ParseError::Expected(Peeked::Numeric, _) => "Expected number".into(),
            ParseError::Expected(Peeked::Colon, _) => "Expected `:` after label".into(),
            ParseError::Expected(Peeked::Comma, _) => "Expected `,` between parameters".into(),
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
            ParseError::Expected(Peeked::EOL, _) => "Found junk at end of line",
            ParseError::Expected(Peeked::Ident, _) => "Expected label or op",
            ParseError::Expected(Peeked::Numeric, _) => "Expected number",
            ParseError::Expected(Peeked::Colon, _) => "Expected `:` after label",
            ParseError::Expected(Peeked::Comma, _) => "Expected `,` between parameters",
        }
    }
}
