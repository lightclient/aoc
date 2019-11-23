use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let floor = count_unmatched_parenthese(&input);
    println!("floor = {}", floor);

    let basement_index = index_of_invalid_parenthese_close(&input);
    println!(
        "basement index = {}",
        basement_index
            .map(|i| (i + 1).to_string())
            .unwrap_or("Did not enter basement".to_string())
    );

    Ok(())
}

fn count_unmatched_parenthese(s: &String) -> i32 {
    let mut count = 0;

    for c in s.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => panic!("invalid character"),
        }
    }

    count
}

fn index_of_invalid_parenthese_close(s: &String) -> Option<usize> {
    let mut count = 0;

    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => {
                if count == 0 {
                    return Some(i);
                } else {
                    count -= 1;
                }
            }
            _ => panic!("invalid character"),
        }
    }

    None
}
