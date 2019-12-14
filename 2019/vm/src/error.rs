use crate::Int;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownInstruction(Int),
    MissingOperand(Int),
    InvalidOperator(Int),
    InvalidProgram,
    EndOfProgram,
}
