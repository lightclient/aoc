use common::load_input;
use std::{cmp::min, collections::HashMap};

type Grid = HashMap<Position, u32>;

const ORIGIN: Position = Position { x: 0, y: 0 };

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

enum Op {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

impl Op {
    pub fn x(&self) -> i32 {
        match self {
            Self::U(_) => 0,
            Self::D(_) => 0,
            Self::L(n) => -n,
            Self::R(n) => *n,
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            Self::U(n) => -n,
            Self::D(n) => *n,
            Self::L(_) => 0,
            Self::R(_) => 0,
        }
    }
}

fn main() {
    let ops: Vec<Vec<String>> =
        load_input!("\n", |s| s.split(',').map(|s| s.to_string()).collect());

    let wire1 = build_grid(parse(ops[0].clone()));
    let wire2 = build_grid(parse(ops[1].clone()));

    let d = solve(wire1, wire2);

    println!("The distance is: {}", d.0);
    println!("The minimum steps is: {}", d.1);
}

fn solve(a: Grid, b: Grid) -> (u32, u32) {
    let mut closest = std::u32::MAX;
    let mut steps = std::u32::MAX;

    for (k, av) in a {
        match b.get(&k) {
            Some(bv) if k != ORIGIN => {
                let d = (i32::abs(k.x) + i32::abs(k.y)) as u32;
                closest = min(d, closest);
                steps = min(av + bv, steps);
            }
            _ => (),
        }
    }

    (closest, steps)
}

fn build_grid(ops: Vec<Op>) -> Grid {
    let mut map = Grid::default();
    let mut pos = Position { x: 0, y: 0 };
    let mut steps = 0;

    for op in ops {
        let x = pos.x;
        let y = pos.y;

        for x in build_range(op.x()) {
            steps += 1;
            map.insert(Position { x: pos.x + x, y }, steps);
        }

        for y in build_range(op.y()) {
            steps += 1;
            map.insert(Position { x, y: pos.y + y }, steps);
        }

        pos.x += op.x();
        pos.y += op.y();
    }

    map
}

fn build_range(n: i32) -> Box<dyn Iterator<Item = i32>> {
    if n < 0 {
        Box::new((n..0).rev())
    } else {
        Box::new(1..=n)
    }
}

fn parse(ops: Vec<String>) -> Vec<Op> {
    let mut ret = vec![];
    for op in ops {
        match op.split_at(1) {
            ("U", rest) => ret.push(Op::U(rest.parse().unwrap())),
            ("D", rest) => ret.push(Op::D(rest.parse().unwrap())),
            ("L", rest) => ret.push(Op::L(rest.parse().unwrap())),
            ("R", rest) => ret.push(Op::R(rest.parse().unwrap())),
            _ => unreachable!(),
        }
    }

    ret
}
