use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position(i32, i32);

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let vists = calc_visits(&input);
    println!("{} houses were visited", vists);

    let vists = calc_alternating_visits(&input);
    println!("{} houses were visited between Santa and Robo-Santa", vists);

    Ok(())
}

fn calc_visits(s: &String) -> u32 {
    let mut position = Position(0, 0);
    let mut visited = HashMap::<Position, u32>::new();
    visited.insert(position, 1);

    for c in s.chars() {
        match c {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            'v' => position.1 -= 1,
            '^' => position.1 += 1,
            '\n' => (),
            _ => panic!("invalid character"),
        }

        *visited.entry(position).or_insert(0) += 1;
    }

    visited.values().len() as u32
}

fn calc_alternating_visits(s: &String) -> u32 {
    let mut a_position = Position(0, 0);
    let mut b_position = Position(0, 0);
    let mut visited = HashMap::<Position, u32>::new();
    visited.insert(a_position, 1);

    for (i, c) in s.chars().enumerate() {
        let position = match i % 2 {
            0 => &mut a_position,
            1 => &mut b_position,
            _ => unreachable!(),
        };

        match c {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            'v' => position.1 -= 1,
            '^' => position.1 += 1,
            _ => panic!("invalid character"),
        }

        *visited.entry(*position).or_insert(0) += 1;
    }

    visited.values().len() as u32
}
