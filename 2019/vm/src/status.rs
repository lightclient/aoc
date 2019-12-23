use crate::Int;

#[derive(Clone, Copy)]
pub enum Status {
    Running,
    Halted,
    Output(Int),
    AwaitingInput,
}
