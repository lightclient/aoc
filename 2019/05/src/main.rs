use common::load_input;
use std::fmt::{self, Display, Formatter};

enum Mode {
    Position(isize),
    Immediate(isize),
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let _ = match self {
            Self::Position(p) => write!(f, "mem({})", p),
            Self::Immediate(i) => write!(f, "\"{}\"", i),
        };

        Ok(())
    }
}

impl Mode {
    pub fn new(value: isize, mode: isize) -> Self {
        match mode {
            0 => Self::Position(value),
            1 => Self::Immediate(value),
            _ => panic!("invalid mode"),
        }
    }
    pub fn get(&self, mem: &Vec<isize>) -> isize {
        match self {
            Self::Position(n) => mem[*n as usize],
            Self::Immediate(n) => *n,
        }
    }
}

fn main() {
    let ops: Vec<isize> = load_input!(',', isize);

    let output1 = process_ops(&ops, 1);
    let output2 = process_ops(&ops, 5);

    println!("Part 1 output: {}", output1);
    println!("Part 2 output: {}", output2);
}

fn process_ops(ops: &[isize], input: isize) -> isize {
    let mut i = 0;
    let mut output = 0;
    let mut mem = ops.to_vec();

    while i < mem.len() {
        let op = mem[i] % 10;

        let mode_c = if mem[i] < 10000 { 1 } else { mem[i] / 10000 };
        let mode_b = (mem[i] % 10000) / 1000;
        let mode_a = ((mem[i] % 10000) % 1000) / 100;

        let mut a = Mode::Position(0);
        let mut b = Mode::Position(0);
        let mut c = Mode::Position(0);

        if i < mem.len() - 3 {
            c = Mode::new(mem[i + 3], mode_c);
        }

        if i < mem.len() - 2 {
            b = Mode::new(mem[i + 2], mode_b);
        }

        if i < mem.len() - 1 {
            a = Mode::new(mem[i + 1], mode_a);
        }

        match op {
            1 | 2 => {
                match op {
                    1 => {
                        let ptr = c.get(&mem) as usize;
                        mem[ptr] = a.get(&mem) + b.get(&mem);
                        println!("add {} {} {}", a, b, c);
                    }
                    2 => {
                        let ptr = c.get(&mem) as usize;
                        mem[ptr] = a.get(&mem) * b.get(&mem);
                        println!("mul {} {} {}", a, b, c);
                    }
                    _ => unreachable!(),
                }

                i += 4;
            }
            3 => {
                println!("write \"{}\" {}", input, a);
                let ptr = mem[i + 1] as usize;
                i += 2;
                mem[ptr] = input;
            }
            4 => {
                println!("out {}", a);
                i += 2;
                output = a.get(&mem);
            }
            5 => {
                println!("je {} {}", a, b);
                i += 3;
                if a.get(&mem) != 0 {
                    i = b.get(&mem) as usize;
                }
            }
            6 => {
                println!("jne {} {}", a, b);
                i += 3;
                if a.get(&mem) == 0 {
                    i = b.get(&mem) as usize;
                }
            }
            7 => {
                println!("jl {} {}", a, b);
                let ptr = c.get(&mem) as usize;
                mem[ptr] = (a.get(&mem) < b.get(&mem)) as isize;
                i += 4;
            }
            8 => {
                println!("eq {} {} {}", a, b, c);
                let ptr = c.get(&mem) as usize;
                mem[ptr] = (a.get(&mem) == b.get(&mem)) as isize;
                i += 4;
            }
            9 => return output,
            _ => panic!("invalid sequence"),
        }
    }

    panic!("invalid program");
}
