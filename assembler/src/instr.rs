use crate::reg::Register;

#[derive(Debug)]
pub enum Instruction {
    Label(String),
    Mov(Register, u32),
}
