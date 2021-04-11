use crate::{error::CodeSpan, parser::ParsedInstruction};
use std::collections::HashMap;

pub type CodegenResult<T> = Result<T, CodegenError>;

pub enum CodegenError {
    LabelAlreadyDefined(CodeSpan),
    ImmediateNotMovable(CodeSpan),
    LabelNotFound(CodeSpan),
}

pub struct Codegen {
    instrs: Vec<ParsedInstruction>,
    symbols: HashMap<String, usize>,
}

impl Codegen {
    pub fn new(instrs: Vec<ParsedInstruction>) -> CodegenResult<Self> {
        let mut codegen = Self {
            instrs,
            symbols: HashMap::new(),
        };
        codegen.gen_symbol_table()?;
        Ok(codegen)
    }

    fn gen_symbol_table(&mut self) -> CodegenResult<()> {
        let mut cur_addr = 0;

        for instr in self.instrs.iter() {
            match instr {
                ParsedInstruction::Label(label) => {
                    if let Some(_) = self.symbols.insert(label.clone(), cur_addr) {
                        return Err(CodegenError::LabelAlreadyDefined(CodeSpan(0, 0)));
                    }
                }
                _ => cur_addr += 4,
            }
        }

        Ok(())
    }

    pub fn gen(&self) -> CodegenResult<Vec<u32>> {
        self.instrs
            .iter()
            .map(|instr| match instr {
                ParsedInstruction::Label(_) => None,
                ParsedInstruction::Mov(reg, imm) => {
                    if imm < &256 {
                        Some(Ok((u32::from(reg) << 12) | (imm << 4)))
                    } else {
                        Some(Err(CodegenError::ImmediateNotMovable(CodeSpan(0, 0))))
                    }
                }
                ParsedInstruction::Jmp(label) => match self.symbols.get(label) {
                    Some(loc) if loc < &256 => Some(Ok((15 << 12) | ((*loc as u32) << 4))),
                    Some(_) => Some(Err(CodegenError::ImmediateNotMovable(CodeSpan(0, 0)))),
                    None => Some(Err(CodegenError::LabelNotFound(CodeSpan(0, 0)))),
                },
                //_ => unimplemented!(),
            })
            .filter_map(|id| id)
            .collect()
    }
}
