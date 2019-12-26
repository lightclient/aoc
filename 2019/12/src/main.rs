use num::integer::lcm;
use std::collections::BTreeSet;
use std::ops::Add;

const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;

const DIMENSIONS: [usize; 3] = [X, Y, Z];

#[derive(Clone, Debug)]
struct Vec3([i64; 3]);

impl Vec3 {
    pub fn abs_sum(&self) -> u64 {
        (self.0[0].abs() + self.0[1].abs() + self.0[2].abs()) as u64
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3([
            self.0[X] + other.0[X],
            self.0[Y] + other.0[Y],
            self.0[Z] + other.0[Z],
        ])
    }
}

#[derive(Clone, Debug)]
struct System {
    position: Vec<Vec3>,
    velocity: Vec<Vec3>,
    pairs: BTreeSet<(usize, usize)>,
}

impl System {
    pub fn new(position: Vec<Vec3>, velocity: Vec<Vec3>) -> Self {
        let mut pairs = BTreeSet::new();
        for a in 0..position.len() {
            for b in 0..position.len() {
                if a != b && !pairs.contains(&(b, a)) {
                    pairs.insert((a, b));
                }
            }
        }

        Self {
            position,
            velocity,
            pairs,
        }
    }

    pub fn energy(&self) -> u64 {
        self.position
            .iter()
            .zip(self.velocity.iter())
            .map(|(a, b)| a.abs_sum() * b.abs_sum())
            .sum()
    }

    pub fn apply_gravity(&mut self, i: usize, a: usize, b: usize) {
        match self.position[a].0[i].cmp(&self.position[b].0[i]) {
            std::cmp::Ordering::Less => {
                self.velocity[a].0[i] += 1;
                self.velocity[b].0[i] -= 1;
            }
            std::cmp::Ordering::Greater => {
                self.velocity[a].0[i] -= 1;
                self.velocity[b].0[i] += 1;
            }
            std::cmp::Ordering::Equal => (),
        }
    }

    pub fn dimension(d: usize, m: &[Vec3], v: &[Vec3]) -> Vec<(i64, i64)> {
        let mut ret = Vec::with_capacity(m.len());
        for (m, v) in m.iter().zip(v.iter()) {
            ret.push((m.0[d], v.0[d]));
        }

        ret
    }
}

struct SystemIterator<'a>(&'a mut System);

impl<'a> Iterator for SystemIterator<'a> {
    type Item = System;

    fn next(&mut self) -> Option<System> {
        // apply gravity
        for p in self.0.pairs.clone() {
            for d in &DIMENSIONS {
                self.0.apply_gravity(*d, p.0, p.1);
            }
        }

        // adjust position
        for i in 0..self.0.position.len() {
            self.0.position[i] = self.0.position[i].clone() + self.0.velocity[i].clone();
        }

        Some(self.0.clone())
    }
}

impl<'a> IntoIterator for &'a mut System {
    type Item = System;
    type IntoIter = SystemIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SystemIterator(self)
    }
}

fn main() {
    let m0 = vec![
        Vec3([-2, 9, -5]),
        Vec3([16, 19, 9]),
        Vec3([0, 3, 6]),
        Vec3([11, 0, 11]),
    ];

    let v0 = vec![Vec3([0, 0, 0]); 4];

    let mut s = System::new(m0.clone(), v0.clone());
    let iter = s.into_iter();
    println!("Part 1: {:?}", iter.skip(999).next().unwrap().energy());

    let (x, y, z) = find_cycles(&m0, &v0);
    println!("Part 2: {}", lcm(x, lcm(y, z)));
}

fn find_cycles(m0: &[Vec3], v0: &[Vec3]) -> (i64, i64, i64) {
    let mut system = System::new(m0.to_vec(), v0.to_vec());
    let mut iter = system.into_iter();

    let d0 = [
        System::dimension(X, m0, v0),
        System::dimension(Y, m0, v0),
        System::dimension(Z, m0, v0),
    ];

    let mut i = 1;
    let mut c = [None; 3];

    while c[X].is_none() || c[Y].is_none() || c[Z].is_none() {
        let s = iter.next().unwrap();

        for dim in DIMENSIONS.iter().cloned() {
            if c[dim].is_none() && System::dimension(dim, &s.position, &s.velocity) == d0[dim] {
                c[dim] = Some(i);
            }
        }

        i += 1;
    }

    (c[X].unwrap(), c[Y].unwrap(), c[Z].unwrap())
}
