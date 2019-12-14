use crate::Int;

#[derive(Debug, PartialEq)]
pub enum Behaviour {
    Read,
    Write,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Relative,
    Immediate,
}

impl Mode {
    pub fn from_raw(n: Int) -> Self {
        match n {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("Invalid mode"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_raw() {
        assert_eq!(Mode::from_raw(0), Mode::Position);
        assert_eq!(Mode::from_raw(1), Mode::Immediate);
        assert_eq!(Mode::from_raw(2), Mode::Relative);

        // TODO: remove panic! and add error
    }
}
