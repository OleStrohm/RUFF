mod error;
mod lexer;
mod parser;
mod reg;
mod codegen;

use std::io::Write;
use crate::parser::ParseLine;
use codegen::Codegen;
use codespan_reporting::{diagnostic::Diagnostic, files::SimpleFiles, term::{
        termcolor::{ColorChoice, StandardStream},
        Config,
    }};
use fs::File;
use std::fs;

fn main() {
    let file = std::env::args().collect::<Vec<_>>().get(1).cloned();
    let file = if let Ok(_) = std::env::var("CARGO") {
        file.unwrap_or("assets/test.rasm".into())
    } else {
        match file {
            Some(file) => file,
            None => {
                eprintln!("No input file supplied");
                return;
            }
        }
    };

    let contents = fs::read_to_string(file).expect("Could not read file");
    let mut files = SimpleFiles::new();

    let file_id = files.add("assets/test.rasm", contents.as_str());

    if let Err(errs) = assemble(contents.clone(), file_id) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
        errs.iter().for_each(|diagnostic| 
            codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &diagnostic)
                .unwrap());
    }
}

fn assemble(contents: String, file_id: usize) -> Result<(), Vec<Diagnostic<usize>>>{
    let (parse_instrs, errors) = contents
        .lines()
        .filter(|l| !l.starts_with("//"))
        .map(|l| (l, ParseLine::new(l)))
        .scan(0, |offset, (line, parse_line)| {
            let res = parse_line.parse();
            let cur_offset = *offset;
            *offset += line.len() + 1;
            Some(((cur_offset, line), res))
        })
        .partition::<Vec<_>, _>(|(_, e)| e.is_ok());

    if !errors.is_empty() {
        return Err(errors.into_iter().map(|((offset, line), err)| {
            err.unwrap_err().diagnostic(offset, line, file_id)
        }).collect());
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

    let instrs: Vec<_> = parse_instrs.into_iter().map(|(_, instr)| instr.unwrap()).collect();

    for instr in instrs.iter() {
        println!("{0: <pad$} | {0:?}", instr, pad = longest_line);
    }

    let codegen = match Codegen::new(instrs).and_then(|cg| cg.gen()) {
        Ok(codegen) => codegen,
        Err(_) => return Err(vec![]),
    };

    let mut out = File::create("out.bin").unwrap();
    
    for (loc, instr) in (0..).map(|i| 4*i).zip(codegen.iter()) {
        println!("{:08X}: {:08X}", loc, instr);

        out.write_all(&instr.to_le_bytes()).unwrap();
    }

    Ok(())
}
