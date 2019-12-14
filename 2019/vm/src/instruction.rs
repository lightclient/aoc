#[derive(Clone, Copy, Debug)]
pub enum Instruction<N> {
    Add(N, N, N),
    Mul(N, N, N),
    Input(N),
    Output(N),
    JumpIfTrue(N, N),
    JumpIfFalse(N, N),
    LessThan(N, N, N),
    Equal(N, N, N),
    AdjustBase(N),
    Return(),
}

impl<N> Instruction<N> {
    pub fn operands(&self) -> Vec<&N> {
        match self {
            Self::Add(a, b, c) => vec![a, b, c],
            Self::Mul(a, b, c) => vec![a, b, c],
            Self::Input(a) => vec![a],
            Self::Output(a) => vec![a],
            Self::JumpIfTrue(a, b) => vec![a, b],
            Self::JumpIfFalse(a, b) => vec![a, b],
            Self::LessThan(a, b, c) => vec![a, b, c],
            Self::Equal(a, b, c) => vec![a, b, c],
            Self::AdjustBase(a) => vec![a],
            Self::Return() => vec![],
        }
    }
}
