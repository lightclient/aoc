use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .collect::<Vec<&str>>();

    let diff = lines.iter().map(char_diff).sum::<u32>();
    let new = lines.iter().map(new_repr).sum::<u32>();

    println!("{} more chars used for representation", diff);
    println!("{} more chars used in new representation", new);

    Ok(())
}

fn char_diff(line: &&str) -> u32 {
    let repr = line.len() as u32;
    let mut data = 0;
    let mut iter = line.chars();

    while let Some(c) = iter.next() {
        match c {
            // we can make the assumption that there will *always* be another character following
            // an initial backslash :-).
            '\\' => match iter.next().unwrap() {
                '\\' | '"' => data += 1,
                'x' => {
                    data += 1;
                    iter.next();
                    iter.next();
                }
                _ => unreachable!(),
            },
            '"' => (),
            _ => data += 1,
        }
    }

    repr - data
}

fn new_repr(line: &&str) -> u32 {
    let data = line.len() as u32;
    let mut new = 0;
    let mut iter = line.chars();

    while let Some(c) = iter.next() {
        match c {
            '\\' => match iter.next().unwrap() {
                '\\' | '"' => new += 4,
                'x' => {
                    new += 5;
                    iter.next();
                    iter.next();
                }
                _ => unreachable!(),
            },
            '"' => new += 2,
            _ => new += 1,
        }
    }

    new - data + 2
}
