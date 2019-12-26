use crate::error::Error;
use crate::op::{Intention, Mode, Op};
use crate::status::Status;
use crate::Int;

#[derive(Clone)]
pub struct Vm {
    pc: usize,
    bp: usize,
    mem: Vec<Int>,
    input: Option<Int>,
    output: Option<Int>,
    code: Vec<Int>,
    status: Status,
}

impl Vm {
    pub fn new(code: &[Int]) -> Self {
        let mut mem = code.to_vec();
        mem.extend(vec![0; 1000]);

        Self {
            pc: 0,
            bp: 0,
            mem,
            input: None,
            output: None,
            code: code.to_vec(),
            status: Status::Running,
        }
    }

    pub fn halted(&self) -> bool {
        match self.status {
            Status::Halted => true,
            _ => false,
        }
    }

    pub fn insert_input(&mut self, input: Int) {
        self.input = Some(input);
    }

    pub fn step(&mut self) -> Result<Status, Error> {
        match self.status {
            Status::Halted => return Err(Error::AlreadyHalted),
            Status::Output(_) => self.status = Status::Running,
            _ => (),
        }

        let op = Op::from_raw(self.get(self.pc));
        self.pc += 1;

        match op {
            Op::Add(ma, mb, mc) => {
                let (a, b, c) = self.params(self.pc, ma, mb, mc);
                self.set(c, a + b);
                self.pc += 3;
            }
            Op::Mul(ma, mb, mc) => {
                let (a, b, c) = self.params(self.pc, ma, mb, mc);
                self.set(c, a * b);
                self.pc += 3;
            }
            Op::Input(ma) => match self.input {
                Some(i) => {
                    let a = self.param(self.pc, ma);
                    self.set(a, i);
                    self.input = None;
                    self.status = Status::Running;
                    self.pc += 1;
                }
                None => {
                    self.status = Status::AwaitingInput;
                    self.pc -= 1;
                }
            },
            Op::Output(ma) => {
                self.output = Some(self.param(self.pc, ma));
                self.status = Status::Output(self.output.unwrap());
                self.pc += 1;
            }
            Op::JumpIfTrue(ma, mb) => {
                let a = self.param(self.pc, ma);
                let b = self.param(self.pc + 1, mb);

                self.pc += 2;

                if a != 0 {
                    self.pc = b as usize;
                }
            }
            Op::JumpIfFalse(ma, mb) => {
                let a = self.param(self.pc, ma);
                let b = self.param(self.pc + 1, mb);

                self.pc += 2;

                if a == 0 {
                    self.pc = b as usize;
                }
            }
            Op::LessThan(ma, mb, mc) => {
                let (a, b, c) = self.params(self.pc, ma, mb, mc);

                if a < b {
                    self.set(c, 1);
                } else {
                    self.set(c, 0);
                }

                self.pc += 3;
            }
            Op::Equal(ma, mb, mc) => {
                let (a, b, c) = self.params(self.pc, ma, mb, mc);

                if a == b {
                    self.set(c, 1);
                } else {
                    self.set(c, 0);
                }

                self.pc += 3;
            }
            Op::AdjustBase(ma) => {
                let a = self.param(self.pc, ma);
                self.bp = ((self.bp as Int) + a) as usize;
                self.pc += 1;
            }
            Op::Halt => self.status = Status::Halted,
        }

        Ok(self.status)
    }

    pub fn run(&mut self, mut inputs: Vec<Int>) -> Option<Int> {
        self.reset();

        loop {
            match self.step() {
                Ok(Status::AwaitingInput) => self.input = Some(inputs.remove(0)),
                Ok(Status::Halted) => break,
                Ok(_) => continue,
                Err(e) => panic!("Error: {:?}", e),
            }
        }

        self.output
    }

    pub fn get(&self, offset: usize) -> Int {
        self.mem[offset]
    }

    pub fn set(&mut self, offset: Int, value: Int) {
        self.mem[offset as usize] = value;
    }

    pub fn reset(&mut self) {
        let mut mem = self.code.to_vec();
        mem.extend(vec![0; 10000]);

        self.pc = 0;
        self.mem = mem;
        self.input = None;
        self.output = None;
        self.status = Status::Running;
    }

    fn params(&self, begin: usize, ma: Intention, mb: Intention, mc: Intention) -> (Int, Int, Int) {
        (
            self.param(begin + 0, ma),
            self.param(begin + 1, mb),
            self.param(begin + 2, mc),
        )
    }

    fn param(&self, p: usize, i: Intention) -> Int {
        match i {
            Intention::Read(m) => match m {
                Mode::Position => self.get(self.get(p) as usize),
                Mode::Immediate => self.get(p),
                Mode::Relative => self.get((self.bp as Int + self.get(p)) as usize),
            },
            Intention::Write(m) => match m {
                Mode::Position => self.get(p),
                Mode::Immediate => panic!("can't write to an immediate"),
                Mode::Relative => self.bp as Int + self.get(p),
            },
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
// }
