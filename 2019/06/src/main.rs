use common::load_input;
use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<(String, String)> = load_input!("\n", |l| {
        let l: Vec<&str> = l.split(")").collect();
        (l[0].to_string(), l[1].to_string())
    });

    let (orbits, orbited) = initialize_graph(&input);

    println!("Orbit count: {}", count_orbits(&orbits, &orbited));
    println!(
        "Distance: {}",
        distance(&orbits, &orbited, "YOU".to_string(), "SAN".to_string())
    );
}

fn count_orbits(orbits: &HashMap<String, String>, orbited: &HashMap<String, Vec<String>>) -> u32 {
    let orbits_keys: HashSet<String> = orbits.keys().cloned().collect();
    let orbits_values: HashSet<String> = orbits.values().cloned().collect();

    let start = orbits_values
        .difference(&orbits_keys)
        .next()
        .expect("graph is not a DAG");

    dfs(start.clone(), &orbited)
}

fn dfs(start: String, orbited: &HashMap<String, Vec<String>>) -> u32 {
    let mut ret = vec![];
    let mut stack = vec![(start, 0)];

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        match orbited.get(&current.0) {
            Some(o) => {
                let count = current.1 + 1;
                let other_paths = o.clone().into_iter().zip(vec![count; o.len()]);
                stack.extend(other_paths);
                ret.push(count - 1);
            }
            None => panic!("invalid parse"),
        }
    }

    ret.iter().sum()
}

fn distance(
    orbits: &HashMap<String, String>,
    orbited: &HashMap<String, Vec<String>>,
    start: String,
    end: String,
) -> u32 {
    let mut visited = HashMap::<String, bool>::new();
    let mut stack = vec![(start, 0)];

    while !stack.is_empty() {
        let current = stack.pop().unwrap();

        if !visited.contains_key(&current.0) {
            visited.insert(current.0.clone(), true);
            let count = current.1 + 1;

            // check the planet it orbits
            if let Some(o) = orbits.get(&current.0) {
                if o.contains(&end) {
                    return count - 2;
                }

                stack.push((o.clone(), count));
            }

            // check the planets it's orbited by
            if let Some(o) = orbited.get(&current.0) {
                let other_paths = o.clone().into_iter().zip(vec![count; o.len()]);
                if o.contains(&end) {
                    return count - 2;
                }
                stack.extend(other_paths);
            }
        }
    }
    panic!("it didn't work");
}

fn initialize_graph(
    input: &Vec<(String, String)>,
) -> (HashMap<String, String>, HashMap<String, Vec<String>>) {
    let mut orbits = HashMap::<String, String>::new();
    let mut orbited = HashMap::<String, Vec<String>>::new();

    for edge in input {
        orbits.insert(edge.1.clone(), edge.0.clone());
        orbited.entry(edge.1.clone()).or_insert(vec![]);
        orbited
            .entry(edge.0.clone())
            .or_insert(vec![])
            .push(edge.1.clone());
    }

    (orbits, orbited)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let input = vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]
        .iter()
        .map(|e| (e.0.into(), e.1.into()))
        .collect();

        let (orbits, orbited) = initialize_graph(&input);
        assert_eq!(count_orbits(&orbits, &orbited), 42);
    }

    #[test]
    fn part2_example() {
        let input = vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
            ("K", "YOU"),
            ("I", "SAN"),
        ]
        .iter()
        .map(|e| (e.0.into(), e.1.into()))
        .collect();

        let (orbits, orbited) = initialize_graph(&input);
        assert_eq!(
            distance(&orbits, &orbited, "YOU".to_string(), "SAN".to_string()),
            6
        );
    }
}
