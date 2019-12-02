use common::load_input;
use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = load_input!("\n");

    let nice_strings_part_one = lines
        .iter()
        .cloned()
        .map(is_nice_part_one)
        .filter(|l| *l)
        .collect::<Vec<bool>>()
        .len();

    let nice_strings_part_two = lines
        .into_iter()
        .map(is_nice_part_two)
        .filter(|l| *l)
        .collect::<Vec<bool>>()
        .len();

    println!(
        "There are {} nice strings for part one.",
        nice_strings_part_one
    );
    println!(
        "There are {} nice strings for part two.",
        nice_strings_part_two
    );
}

fn is_nice_part_one(s: String) -> bool {
    let mut double = false;
    let mut bad_substring = false;
    let mut last = s.chars().next().unwrap();
    let mut vowels = is_vowel(last) as u32;

    for c in s.chars().skip(1) {
        if is_vowel(c) {
            vowels += 1;
        }

        match (last, c) {
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => bad_substring = true,
            _ => (),
        }

        if last == c {
            double = true;
        }

        last = c;
    }

    vowels >= 3 && double && !bad_substring
}

fn is_nice_part_two(s: String) -> bool {
    let mut repeat = false;
    let mut last_last = s.chars().next().unwrap();
    let mut last = s.chars().skip(1).next().unwrap();
    let mut last_pair = (last_last, last);
    let mut pairs = HashMap::<(char, char), u32>::new();
    pairs.insert(last_pair, 1);

    for c in s.chars().skip(2) {
        let current_pair = (last, c);

        if current_pair != last_pair {
            *pairs.entry(current_pair).or_insert(0) += 1
        }

        if last_last == c {
            repeat = true;
        }

        last_pair = current_pair;
        last_last = last;
        last = c;
    }

    let not_overlapping_pairs = pairs
        .values()
        .cloned()
        .filter(|v| *v >= 2)
        .collect::<Vec<u32>>()
        .len();

    not_overlapping_pairs > 0 && repeat
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
