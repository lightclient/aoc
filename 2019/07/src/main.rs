use common::load_input;
use std::fmt::{self, Display, Formatter};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

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

    let permutations1 = heaps_alg(vec![0, 1, 2, 3, 4]);
    let permutations2 = heaps_alg(vec![5, 6, 7, 8, 9]);

    let mut part1 = vec![];
    let mut part2 = vec![];

    for p in permutations1 {
        part1.push(run(&ops, p));
    }

    for p in permutations2 {
        part2.push(run(&ops, p));
    }

    println!("Part 1 output: {}", part1.iter().max().unwrap());
    println!("Part 2 output: {}", part2.iter().max().unwrap());
}

fn heaps_alg(v: Vec<isize>) -> Vec<Vec<isize>> {
    fn f(len: usize, v: &mut Vec<isize>, ret: &mut Vec<Vec<isize>>) {
        if len == 0 || len == 1 {
            ret.push(v.to_owned());
            return;
        }

        for i in 0..len - 1 {
            f(len - 1, v, ret);
            let j = if len % 2 == 0 { i } else { 0 };
            v.swap(j, len - 1);
        }

        f(len - 1, v, ret);
    }

    let mut ret = vec![];
    let mut v = v.clone();

    f(v.len(), &mut v, &mut ret);

    ret
}

fn run(ops: &[isize], settings: Vec<isize>) -> isize {
    let mut ret = 0;

    let (tx_a, rx_a): (Sender<isize>, Receiver<isize>) = mpsc::channel();
    let (tx_b, rx_b): (Sender<isize>, Receiver<isize>) = mpsc::channel();
    let (tx_c, rx_c): (Sender<isize>, Receiver<isize>) = mpsc::channel();
    let (tx_d, rx_d): (Sender<isize>, Receiver<isize>) = mpsc::channel();
    let (tx_e, rx_e): (Sender<isize>, Receiver<isize>) = mpsc::channel();

    let _ = crossbeam::scope(|s| {
        tx_e.send(0).expect("can't initialize a");

        let a = s.spawn(|_| process_ops(&ops, settings[0], tx_a, rx_e));
        let b = s.spawn(|_| process_ops(&ops, settings[1], tx_b, rx_a));
        let c = s.spawn(|_| process_ops(&ops, settings[2], tx_c, rx_b));
        let d = s.spawn(|_| process_ops(&ops, settings[3], tx_d, rx_c));
        let e = s.spawn(|_| process_ops(&ops, settings[4], tx_e, rx_d));

        a.join().expect("unable to join");
        b.join().expect("unable to join");
        c.join().expect("unable to join");
        d.join().expect("unable to join");

        ret = e.join().expect("unable to join");
    });

    ret
}

fn process_ops(ops: &[isize], setting: isize, tx: Sender<isize>, rx: Receiver<isize>) -> isize {
    let mut mem = ops.to_vec();
    let mut initialized = false;
    let mut output = 0;
    let mut i = 0;

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
                        println!("add {} {} {}\t;; {}", a, b, c, mem[ptr]);
                    }
                    2 => {
                        let ptr = c.get(&mem) as usize;
                        mem[ptr] = a.get(&mem) * b.get(&mem);
                        println!("mul {} {} {}\t\t;; {}", a, b, c, mem[ptr]);
                    }
                    _ => unreachable!(),
                }

                i += 4;
            }
            3 => {
                let input = if !initialized {
                    initialized = true;
                    setting
                } else {
                    rx.recv().unwrap()
                };

                println!("in \"{}\" {}", input, a);
                let ptr = mem[i + 1] as usize;
                i += 2;
                mem[ptr] = input;
            }
            4 => {
                i += 2;
                output = a.get(&mem);
                println!("out {} \t\t\t;; {}", a, output);
                let _ = tx.send(output);
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
