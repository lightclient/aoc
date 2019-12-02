use common::load_input;

fn main() {
    let input = load_input!();
    let floor = count_unmatched_parenthese(&input);
    let basement_index = index_of_invalid_parenthese_close(&input);

    println!("floor = {}", floor);
    println!(
        "basement index = {}",
        basement_index
            .map(|i| (i + 1).to_string())
            .unwrap_or("Did not enter basement".to_string())
    );
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
