use common::load_input;
use std::collections::HashMap;
use vm::{Int, Status, Vm};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: i64,
    y: i64,
}

struct Bot {
    d: Direction,
    p: Position,
}

impl Bot {
    pub fn raw_turn(&mut self, n: i64) {
        self.d = match n {
            0 => match self.d {
                Direction::U => Direction::L,
                Direction::D => Direction::R,
                Direction::L => Direction::D,
                Direction::R => Direction::U,
            },
            1 => match self.d {
                Direction::U => Direction::R,
                Direction::D => Direction::L,
                Direction::L => Direction::U,
                Direction::R => Direction::D,
            },
            _ => panic!("invalid turn: {}", n),
        }
    }

    pub fn walk(&mut self, n: i64) {
        match self.d {
            Direction::U => self.p.y += n,
            Direction::D => self.p.y -= n,
            Direction::L => self.p.x -= n,
            Direction::R => self.p.x += n,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    U,
    D,
    L,
    R,
}

fn main() {
    let code: Vec<Int> = load_input!(",", Int);
    let mut part1: HashMap<Position, Int> = HashMap::new();
    let mut part2: HashMap<Position, Int> = HashMap::new();

    paint(&code, &mut part1, 0);
    paint(&code, &mut part2, 1);
    display(&part2);

    println!("painted at least: {}", part1.len());
}

fn paint(code: &[Int], map: &mut HashMap<Position, Int>, initial: Int) {
    let mut vm = Vm::new(&code);
    vm.insert_input(initial);

    let mut current = Bot {
        d: Direction::U,
        p: Position { x: 0, y: 0 },
    };

    let mut outputs = vec![];

    loop {
        let color = map.entry(current.p).or_insert(0);

        match vm.step() {
            Ok(Status::AwaitingInput) => vm.insert_input(*color),
            Ok(Status::Output(o)) => outputs.push(o),
            Ok(Status::Halted) => break,
            Ok(_) => continue,
            Err(e) => panic!("{:?}", e),
        }

        if outputs.len() == 2 {
            *color = outputs[0];
            current.raw_turn(outputs[1]);
            current.walk(1);
            outputs.clear();
        }
    }
}

fn display(map: &HashMap<Position, Int>) {
    let mut max_u = std::i64::MIN;
    let mut max_d = std::i64::MAX;
    let mut max_l = std::i64::MAX;
    let mut max_r = std::i64::MIN;

    for p in map.keys() {
        if p.x < max_l {
            max_l = p.x
        }
        if p.x > max_r {
            max_r = p.x
        }
        if p.y > max_u {
            max_u = p.y
        }
        if p.y < max_d {
            max_d = p.y
        }
    }

    println!("d: {}, u: {}, l: {}, r: {}", max_d, max_u, max_l, max_r);
    for y in (max_d..max_u + 1).rev() {
        let mut row = String::new();

        for x in max_l..max_r + 1 {
            match map.get(&Position { x, y }) {
                Some(1) => row.push('#'),
                _ => row.push(' '),
            }
        }

        println!("{}", row);
    }
}
