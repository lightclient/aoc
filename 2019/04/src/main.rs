fn main() {
    let range = 246515..=739105;

    let part1 = range
        .clone()
        .map(|v| is_valid(v))
        .filter(|v| *v)
        .collect::<Vec<bool>>()
        .len();

    let part2 = range
        .map(|v| is_valid(v) && exactly_two(v))
        .filter(|v| *v)
        .collect::<Vec<bool>>()
        .len();

    println!("The number of valid values for part 1 is {}", part1);
    println!("The number of valid values for part 2 is {}", part2);
}

fn exactly_two(n: u32) -> bool {
    let n: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let len = n.len();
    let first_two = n[0] == n[1] && n[1] != n[2];
    let last_two = n[len - 1] == n[len - 2] && n[len - 2] != n[len - 3];

    if first_two || last_two {
        return true;
    }

    let mut double = false;

    for i in 3..n.len() {
        if n[i - 3] != n[i - 2] && n[i - 2] == n[i - 1] && n[i] != n[i - 1] {
            double = true;
        }
    }

    double
}

fn is_valid(n: u32) -> bool {
    if n < 100000 || n > 999999 {
        return false;
    }

    let n: Vec<u32> = n
        .to_string()
        .chars()
        .map(|v| v.to_string().parse().unwrap())
        .collect();

    let mut flag = false;
    let mut last = 0u32;
    for d in n {
        if d < last {
            return false;
        }

        if d == last {
            flag = true;
        }

        last = d;
    }

    flag
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(is_valid(111111), true);
        assert_eq!(exactly_two(111111), false);
        assert_eq!(exactly_two(112233), true);
        assert_eq!(exactly_two(123444), false);
        assert_eq!(exactly_two(111122), true);
        assert_eq!(exactly_two(331111), true);
    }
}
