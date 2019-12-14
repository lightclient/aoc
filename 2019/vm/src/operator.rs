use crate::error::Error;
use crate::operand::Mode;
use crate::Int;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equal(Mode, Mode, Mode),
    AdjustBase(Mode),
    Return(),
}

impl Operator {
    pub fn from_raw(n: Int) -> Result<Self, Error> {
        let a = Mode::from_raw(((n % 10000) % 1000) / 100);
        let b = Mode::from_raw((n % 10000) / 1000);
        let c = Mode::from_raw(if n < 10000 { 1 } else { n / 10000 });

        let ret = match n % 100 {
            1 => Operator::Add(a, b, c),
            2 => Operator::Mul(a, b, c),
            3 => Operator::Input(if n < 100 { Mode::Immediate } else { a }),
            4 => Operator::Output(a),
            5 => Operator::JumpIfTrue(a, b),
            6 => Operator::JumpIfFalse(a, b),
            7 => Operator::LessThan(a, b, c),
            8 => Operator::Equal(a, b, c),
            9 => Operator::AdjustBase(a),
            99 => Operator::Return(),
            n => return Err(Error::InvalidOperator(n % 10)),
        };

        Ok(ret)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_raw() {
        assert_eq!(
            Operator::from_raw(1),
            Ok(Operator::Add(
                Mode::Position,
                Mode::Position,
                Mode::Immediate
            ))
        );
        assert_eq!(
            Operator::from_raw(103),
            Ok(Operator::Input(Mode::Immediate))
        );
        assert_eq!(
            Operator::from_raw(2005),
            Ok(Operator::JumpIfTrue(Mode::Position, Mode::Relative))
        );
        assert_eq!(
            Operator::from_raw(10208),
            Ok(Operator::Equal(
                Mode::Relative,
                Mode::Position,
                Mode::Immediate
            ))
        );
    }
}
