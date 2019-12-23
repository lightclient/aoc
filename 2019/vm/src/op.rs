use crate::Int;

#[derive(Debug, PartialEq)]
pub enum Intention {
    Read(Mode),
    Write(Mode),
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    pub fn from_raw(n: Int) -> Self {
        match n {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("invalid mode: {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add(Intention, Intention, Intention),
    Mul(Intention, Intention, Intention),
    Input(Intention),
    Output(Intention),
    JumpIfTrue(Intention, Intention),
    JumpIfFalse(Intention, Intention),
    LessThan(Intention, Intention, Intention),
    Equal(Intention, Intention, Intention),
    AdjustBase(Intention),
    Halt,
}

macro_rules! r {
    ($m:ident) => {
        Intention::Read($m)
    };
}

macro_rules! w {
    ($m:ident) => {
        Intention::Write($m)
    };
}

impl Op {
    pub fn from_raw(n: Int) -> Self {
        let raw = n % 100;

        let a = Mode::from_raw((n / 100) % 10);
        let b = Mode::from_raw((n / 1000) % 10);
        let c = Mode::from_raw((n / 10000) % 10);

        match raw {
            1 => Op::Add(r!(a), r!(b), w!(c)),
            2 => Op::Mul(r!(a), r!(b), w!(c)),
            3 => Op::Input(w!(a)),
            4 => Op::Output(r!(a)),
            5 => Op::JumpIfTrue(r!(a), r!(b)),
            6 => Op::JumpIfFalse(r!(a), r!(b)),
            7 => Op::LessThan(r!(a), r!(b), w!(c)),
            8 => Op::Equal(r!(a), r!(b), w!(c)),
            9 => Op::AdjustBase(r!(a)),
            99 => Op::Halt,
            _ => panic!("unknown opcode: {}", raw),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Op::from_raw(1),
            Op::Add(
                Intention::Read(Mode::Position),
                Intention::Read(Mode::Position),
                Intention::Write(Mode::Position)
            )
        );

        assert_eq!(
            Op::from_raw(102),
            Op::Mul(
                Intention::Read(Mode::Immediate),
                Intention::Read(Mode::Position),
                Intention::Write(Mode::Position)
            )
        );

        assert_eq!(
            Op::from_raw(1002),
            Op::Mul(
                Intention::Read(Mode::Position),
                Intention::Read(Mode::Immediate),
                Intention::Write(Mode::Position)
            )
        );

        assert_eq!(
            Op::from_raw(1102),
            Op::Mul(
                Intention::Read(Mode::Immediate),
                Intention::Read(Mode::Immediate),
                Intention::Write(Mode::Position)
            )
        );
    }
}
