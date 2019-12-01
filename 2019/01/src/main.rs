use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let masses: Vec<u32> = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| l.parse::<u32>().expect("must be valid number"))
        .collect();

    let fuel: u32 = masses.clone().into_iter().map(calc_fuel).sum();
    let recursive_fuel: u32 = masses.into_iter().map(recursively_calc_fuel).sum();

    println!("The fuel required is {}", fuel);
    println!("The fuel recursively required is {}", recursive_fuel);

    Ok(())
}

fn calc_fuel(n: u32) -> u32 {
    (n / 3) - 2
}

fn recursively_calc_fuel(n: u32) -> u32 {
    if n == 0 {
        return n;
    }

    let (n, overflow) = (n / 3).overflowing_sub(2);
    let n = if !overflow { n } else { 0 };

    n + recursively_calc_fuel(n)
}
