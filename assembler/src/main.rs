mod error;
mod instr;
mod lexer;
mod parser;
mod reg;

use crate::parser::ParseLine;
use codespan_reporting::{
    files::SimpleFiles,
    term::{
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use std::fs;

fn main() {
    let contents = fs::read_to_string("assets/test.rasm").expect("Could not read file");

    let mut files = SimpleFiles::new();

    let file_id = files.add("assets/test.rasm", contents.as_str());

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();

    let (parse_instrs, errors) = contents
        .lines()
        .map(|l| (l, ParseLine::new(l)))
        .scan(0, |offset, (line, parse_line)| {
            let res = parse_line.parse();
            let cur_offset = *offset;
            *offset += line.len() + 1;
            Some(((cur_offset, line), res))
        })
        .partition::<Vec<_>, _>(|(_, e)| e.is_ok());

    if !errors.is_empty() {
        for ((offset, line), err) in errors {
            let err = err.unwrap_err();
            let diagnostic = err.diagnostic(offset, line, file_id);
            codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &diagnostic)
                .unwrap();
        }
        return;
    }

    let longest_line = parse_instrs
        .iter()
        .map(|(_, instr)| {
            let instr = instr.as_ref().unwrap();
            let line = format!("{}", instr);
            line.chars().count()
        })
        .max()
        .unwrap();

    for (_, instr) in parse_instrs {
        let instr = instr.unwrap();
        println!("{0: <pad$} | {0:?}", instr, pad = longest_line);
    }
}
