use common::load_input;

fn main() {
    let masses: Vec<u32> = load_input!('\n', |l| l.parse().unwrap());

    let fuel: u32 = masses.clone().into_iter().map(calc_fuel).sum();
    let recursive_fuel: u32 = masses.into_iter().map(recursively_calc_fuel).sum();

    println!("The fuel required is {}", fuel);
    println!("The fuel recursively required is {}", recursive_fuel);
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
