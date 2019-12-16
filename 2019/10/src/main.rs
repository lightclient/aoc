use common::load_input;
use ordered_float::OrderedFloat;
use std::collections::HashMap;

type Rad = OrderedFloat<f64>;

#[derive(Clone, PartialEq)]
enum Element {
    Empty,
    Astroid,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    r: i64,
    c: i64,
}

fn main() {
    let lines: Vec<String> = load_input!('\n', String);
    let mem = parse_input(&lines);

    println!("Most visible = {}", solve1(&mem));
    println!("200th astroid = {:?}", solve2(&mem, 200));
}

fn solve1(map: &HashMap<Position, HashMap<Rad, Vec<Position>>>) -> usize {
    let mut best = 0;

    for p in map.values() {
        if best < p.len() {
            best = p.len();
        }
    }

    best
}

fn solve2(map: &HashMap<Position, HashMap<Rad, Vec<Position>>>, rounds: usize) -> usize {
    let station = map.iter().max_by(|x, y| x.1.len().cmp(&y.1.len())).unwrap();
    let sp = station.0.clone();
    let mut sm = station.1.clone();
    let mut angles: Vec<Rad> = sm.keys().cloned().collect();

    // sort largest to smallest
    angles.sort_by(|a, b| {
        let a: f64 = (a.clone()).into();
        let b: f64 = (b.clone()).into();
        b.partial_cmp(&a).unwrap()
    });

    // Q1 => 0 < r <= pi/2
    let first: Vec<Rad> = angles
        .clone()
        .into_iter()
        .filter(|r| OrderedFloat(0f64) < *r && *r <= OrderedFloat(std::f64::consts::FRAC_PI_2))
        .collect();

    // Q2 => pi / 2 < r <= pi
    let second: Vec<Rad> = angles
        .clone()
        .into_iter()
        .filter(|r| {
            OrderedFloat(std::f64::consts::FRAC_PI_2) < *r
                && *r <= OrderedFloat(std::f64::consts::PI)
        })
        .collect();

    // Q3 & Q4 => -pi < r <= 0
    let three_four: Vec<Rad> = angles
        .clone()
        .into_iter()
        .filter(|r| OrderedFloat(-std::f64::consts::PI) < *r && *r <= OrderedFloat(0f64))
        .collect();

    let mut rads = vec![];

    rads.extend(first);
    rads.extend(three_four);
    rads.extend(second);

    let mut i = 0;
    let mut vaporized = Position::default();

    for _ in 0..rounds {
        let angle = &rads[i % rads.len()];
        let astroids = sm.get_mut(angle).unwrap();

        // bad to do this each, but ¯\_(ツ)_/¯
        astroids.sort_by(|a, b| ((sp.r - a.r) + (a.c - sp.c)).cmp(&((sp.r - b.r) + (b.c - sp.c))));

        vaporized = astroids.remove(0);

        if astroids.is_empty() {
            rads.remove(i % rads.len());

            // don't overflow a usize
            if i == 0 {
                i = rads.len() - 1;
            } else {
                i -= 1
            }
        }

        i = (i + 1) % rads.len();
    }

    (vaporized.c * 100 + vaporized.r) as usize
}

fn map_astroids_to_angle(p: Position, astroids: &Vec<Position>) -> HashMap<Rad, Vec<Position>> {
    let mut angles: HashMap<Rad, Vec<Position>> = HashMap::new();

    for a in astroids {
        let rel = Position {
            r: p.r - a.r,
            c: a.c - p.c,
        };

        let x = rel.c as f64;
        let y = rel.r as f64;

        let rad = y.atan2(x);

        let angle = angles.entry(rad.into()).or_insert(vec![]);
        angle.push(*a);
    }

    angles
}

fn parse_input(lines: &Vec<String>) -> HashMap<Position, HashMap<Rad, Vec<Position>>> {
    let mut map: HashMap<Position, Element> = HashMap::new();

    for (i, r) in lines.iter().enumerate() {
        let i = i as i64;
        for (j, c) in r.chars().enumerate() {
            let j = j as i64;
            match c {
                '.' => map.insert(Position { r: i, c: j }, Element::Empty),
                '#' => map.insert(Position { r: i, c: j }, Element::Astroid),
                _ => panic!("Invalid input"),
            };
        }
    }

    let astroids: Vec<Position> = map
        .clone()
        .into_iter()
        .filter(|(_, v)| *v == Element::Astroid)
        .map(|(k, _)| k)
        .collect();

    let mut mem: HashMap<Position, HashMap<Rad, Vec<Position>>> = HashMap::new();
    for p in astroids.clone() {
        mem.insert(p, map_astroids_to_angle(p, &astroids));
    }

    mem
}

#[cfg(test)]
mod test {
    use super::*;

    const BIG: &str = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#;

    #[test]
    fn part2_simple_example() {
        let input = vec![".#..#", ".....", "#####", "....#", "...##"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let map = parse_input(&input);
        assert_eq!(solve1(&map), 8);
    }

    #[test]
    fn part1_larger_example() {
        let input = BIG.split("\n").map(|s| s.to_string()).collect();
        let map = parse_input(&input);
        assert_eq!(solve1(&map), 210);
    }

    #[test]
    fn part2_smaller_example() {
        let input = r#".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##"#
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        let map = parse_input(&input);
        assert_eq!(solve2(&map, 36), 1303);
    }

    #[test]
    fn part2_larger_example() {
        let input = BIG.split("\n").map(|s| s.to_string()).collect();

        let map = parse_input(&input);
        assert_eq!(solve2(&map, 200), 802);
    }
}
