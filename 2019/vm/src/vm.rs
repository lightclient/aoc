use crate::error::Error;
use crate::instruction::Instruction;
use crate::operand::{Behaviour, Mode};
use crate::operator::Operator;
use crate::Int;
use slog::{o, trace, Drain, Logger};
use std::sync::mpsc::{Receiver, Sender};

enum Input<T> {
    List(Vec<T>),
    Channel(Receiver<T>),
}

enum Output<T> {
    List(Vec<T>),
    Channel(Sender<T>),
}

pub struct Vm {
    pc: usize,
    bp: usize,
    mem: Vec<Int>,
    input: Input<Int>,
    output: Output<Int>,
    logger: Logger,
}

impl Vm {
    pub fn new(mut mem: Vec<Int>, input: Vec<Int>) -> Self {
        mem.extend(vec![0; 1000]);

        Self {
            pc: 0,
            bp: 0,
            mem,
            input: Input::List(input),
            output: Output::List(vec![]),
            logger: Self::init_logger(),
        }
    }

    pub fn from_channels(mem: Vec<Int>, input: Receiver<Int>, output: Sender<Int>) -> Self {
        Self {
            pc: 0,
            bp: 0,
            mem,
            input: Input::Channel(input),
            output: Output::Channel(output),
            logger: Self::init_logger(),
        }
    }

    fn init_logger() -> Logger {
        let decorator = slog_term::PlainDecorator::new(std::io::stdout());
        let drain = slog_term::CompactFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).chan_size(1000).build().fuse();

        slog::Logger::root(drain, o!())
    }

    pub fn push_input(&mut self, input: Int) {
        match &mut self.input {
            Input::List(l) => l.push(input),
            Input::Channel(_) => unimplemented!(),
        }
    }

    fn set(&mut self, ptr: Int, val: Int) {
        self.mem[ptr as usize] = val;
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.bp = 0;
    }

    fn next_instruction(&mut self) -> Option<Instruction<Int>> {
        let instruction = self.parse_instruction(self.pc);

        if let Ok(instruction) = instruction {
            self.pc += 1 + instruction.operands().len();
        }

        instruction.ok()
    }

    fn load_operand(&self, n: Int, mode: Mode, behaviour: Behaviour) -> Option<Int> {
        match mode {
            Mode::Immediate => Some(n),
            Mode::Position => match behaviour {
                Behaviour::Read => self.mem.get(n as usize).map(|v| *v),
                Behaviour::Write => Some(n),
            },
            Mode::Relative => match behaviour {
                Behaviour::Read => self.mem.get((self.bp as Int + n) as usize).map(|v| *v),
                Behaviour::Write => Some(self.bp as Int + n),
            },
        }
    }

    fn parse_instruction(&self, index: usize) -> Result<Instruction<Int>, Error> {
        let raw = self.mem[index];
        let op = Operator::from_raw(raw)?;

        let a = self.mem.get(index + 1).map(|v| *v);
        let b = self.mem.get(index + 2).map(|v| *v);
        let c = self.mem.get(index + 3).map(|v| *v);

        let instruction = match op {
            Operator::Add(ma, mb, mc) => Instruction::Add(
                a.and_then(|n| self.load_operand(n, ma, Behaviour::Read))
                    .ok_or(Error::MissingOperand(1))?,
                b.and_then(|n| self.load_operand(n, mb, Behaviour::Read))
                    .ok_or(Error::MissingOperand(2))?,
                c.and_then(|n| self.load_operand(n, mc, Behaviour::Write))
                    .ok_or(Error::MissingOperand(3))?,
            ),
            Operator::Mul(ma, mb, mc) => Instruction::Mul(
                a.and_then(|n| self.load_operand(n, ma, Behaviour::Read))
                    .ok_or(Error::MissingOperand(1))?,
                b.and_then(|n| self.load_operand(n, mb, Behaviour::Read))
                    .ok_or(Error::MissingOperand(2))?,
                c.and_then(|n| self.load_operand(n, mc, Behaviour::Write))
                    .ok_or(Error::MissingOperand(3))?,
            ),
            Operator::Input(ma) => Instruction::Input(
                a.and_then(|n| self.load_operand(n, ma, Behaviour::Write))
                    .ok_or(Error::MissingOperand(1))?,
            ),

            Operator::Output(ma) => Instruction::Output(
                a.and_then(|n| self.load_operand(n, ma, Behaviour::Read))
                    .ok_or(Error::MissingOperand(1))?,
            ),
            Operator::JumpIfTrue(ma, mb) => Instruction::JumpIfTrue(
                a.and_then(|n| self.load_operand(n, ma, Behaviour::Read))
                    .ok_or(Error::MissingOperand(1))?,
                b.and_then(|n| self.load_operand(n, mb, Behaviour::Read))
                    .ok_or(Error::MissingOperand(2))?,
            ),
            Operator::JumpIfFalse(ma, mb) => Instruction::JumpIfFalse(
                a.and_then(|n| self.load_operand(n, ma, Behaviour::Read))
                    .ok_or(Error::MissingOperand(1))?,
                b.and_then(|n| self.load_operand(n, mb, Behaviour::Read))
                    .ok_or(Error::MissingOperand(2))?,
            ),
            Operator::LessThan(ma, mb, mc) => Instruction::LessThan(
                a.and_then(|n| self.load_operand(n, ma, Behaviour::Read))
                    .ok_or(Error::MissingOperand(1))?,
                b.and_then(|n| self.load_operand(n, mb, Behaviour::Read))
                    .ok_or(Error::MissingOperand(2))?,
                c.and_then(|n| self.load_operand(n, mc, Behaviour::Write))
                    .ok_or(Error::MissingOperand(3))?,
            ),
            Operator::Equal(ma, mb, mc) => Instruction::Equal(
                a.and_then(|n| self.load_operand(n, ma, Behaviour::Read))
                    .ok_or(Error::MissingOperand(1))?,
                b.and_then(|n| self.load_operand(n, mb, Behaviour::Read))
                    .ok_or(Error::MissingOperand(2))?,
                c.and_then(|n| self.load_operand(n, mc, Behaviour::Write))
                    .ok_or(Error::MissingOperand(3))?,
            ),
            Operator::AdjustBase(ma) => Instruction::AdjustBase(
                a.and_then(|n| self.load_operand(n, ma, Behaviour::Read))
                    .ok_or(Error::MissingOperand(1))?,
            ),
            Operator::Return() => Instruction::Return(),
        };

        Ok(instruction)
    }

    pub fn step(&mut self) -> Result<(), Error> {
        if let Some(op) = self.next_instruction() {
            match op {
                Instruction::Add(a, b, c) => {
                    self.set(c, a + b);
                    trace!(
                        self.logger,
                        "add {} {} {}\t;; {}",
                        a,
                        b,
                        c,
                        self.mem[c as usize]
                    );
                }
                Instruction::Mul(a, b, c) => {
                    self.set(c, a * b);
                    trace!(
                        self.logger,
                        "mul {} {} {}\t\t;; {}",
                        a,
                        b,
                        c,
                        self.mem[c as usize]
                    );
                }
                Instruction::Input(a) => {
                    let input = match &mut self.input {
                        Input::List(i) => i.remove(0),
                        Input::Channel(c) => c.recv().unwrap(),
                    };

                    trace!(self.logger, "in \"{}\" {}", input, a);

                    self.set(a, input);
                }
                Instruction::Output(a) => {
                    match &mut self.output {
                        Output::List(o) => o.push(a),
                        Output::Channel(c) => c.send(a).unwrap(),
                    };

                    trace!(self.logger, "out {}", a);
                }
                Instruction::JumpIfTrue(a, b) => {
                    if a == 1 {
                        self.pc = b as usize;
                    }
                    trace!(self.logger, "je {} {}", a, b);
                }
                Instruction::JumpIfFalse(a, b) => {
                    if a == 0 {
                        self.pc = b as usize;
                    }
                    trace!(self.logger, "jne {} {}", a, b);
                }
                Instruction::LessThan(a, b, c) => {
                    self.set(c, (a < b) as Int);
                    trace!(self.logger, "jl {} {}", a, b);
                }
                Instruction::Equal(a, b, c) => {
                    self.set(c, (a == b) as Int);
                    trace!(self.logger, "eq {} {} {}", a, b, c);
                }
                Instruction::AdjustBase(a) => {
                    if a < 0 {
                        self.bp -= Int::abs(a) as usize;
                    } else {
                        self.bp += a as usize;
                    }
                    trace!(self.logger, "adjb {}", a);
                }
                Instruction::Return() => {
                    trace!(self.logger, "ret");
                    return Err(Error::EndOfProgram);
                }
            };

            Ok(())
        } else {
            Err(Error::InvalidProgram)
        }
    }

    pub fn run(&mut self) -> Option<&[Int]> {
        self.reset();

        loop {
            match self.step() {
                Ok(_) => continue,
                Err(Error::EndOfProgram) => break,
                Err(e) => panic!("{:?}", e),
            }
        }

        match &self.output {
            Output::List(l) => Some(l),
            Output::Channel(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equal_position() {
        let code = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut vm = Vm::new(code, vec![8]);
        assert_eq!(vm.run(), Some(&[1][..]));
    }

    #[test]
    fn less_than_position() {
        let code = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut vm = Vm::new(code, vec![8]);
        assert_eq!(vm.run(), Some(&[0][..]));
    }

    #[test]
    fn equal_to_immediate() {
        let code = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut vm = Vm::new(code, vec![1]);
        assert_eq!(vm.run(), Some(&[0][..]));
    }

    #[test]
    fn less_than_immediate() {
        let code = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut vm = Vm::new(code, vec![6]);
        assert_eq!(vm.run(), Some(&[1][..]));
    }

    #[test]
    fn small_program() {
        let code = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut vm = Vm::new(code.clone(), vec![7]);
        assert_eq!(vm.run(), Some(&[999][..]));

        let mut vm = Vm::new(code, vec![9]);
        assert_eq!(vm.run(), Some(&[1001][..]));
    }

    #[test]
    fn thruster_example() {
        let code = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];

        let mut vm = Vm::new(code, vec![]);

        // a
        vm.push_input(1);
        vm.push_input(0);
        let mut ret = vm.run().unwrap().to_vec();

        // b
        vm.push_input(0);
        vm.push_input(ret[0]);
        ret = vm.run().unwrap().to_vec();

        // c
        vm.push_input(4);
        vm.push_input(ret[1]);
        ret = vm.run().unwrap().to_vec();

        // d
        vm.push_input(3);
        vm.push_input(ret[2]);
        ret = vm.run().unwrap().to_vec();

        // e
        vm.push_input(2);
        vm.push_input(ret[3]);
        assert_eq!(vm.run().unwrap()[4], 65210);
    }

    #[test]
    fn quine() {
        let code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut vm = Vm::new(code.clone(), vec![9]);
        assert_eq!(vm.run(), Some(&code[..]));
    }

    #[test]
    fn mul_large_numbers() {
        let code = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut vm = Vm::new(code.clone(), vec![0]);
        assert_eq!(vm.run(), Some(&[1219070632396864][..]));
    }

    #[test]
    fn ret_large_number() {
        let code = vec![104, 1125899906842624, 99];
        let mut vm = Vm::new(code.clone(), vec![0]);
        assert_eq!(vm.run(), Some(&[1125899906842624][..]));
    }

    #[test]
    fn diagnostic() {
        let input = std::fs::read_to_string("../09/input/input.txt").unwrap();

        let code: Vec<Int> = input
            .split(",")
            .map(|l| (l.replace(&['\n'][..], "")).parse().unwrap())
            .collect();

        let mut vm = Vm::new(code.clone(), vec![1]);
        assert_eq!(vm.run(), Some(&[2453265701][..]));
    }
}
