use common::load_input;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const AREA: usize = WIDTH * HEIGHT;

fn main() {
    let input: Vec<usize> = load_input!("", usize);

    let part1 = checksum(&input, AREA);
    let part2 = combine_layers(&input, AREA);

    println!("The solution to part 1 is {}", part1);
    println!("The resulting image is \n{}", show(&part2, WIDTH));
}

fn checksum(input: &Vec<usize>, area: usize) -> usize {
    let layers = input.chunks(area);

    let least_zeros = layers
        .min_by(|x, y| {
            let x_0s = x.iter().filter(|x| **x == 0).count();
            let y_0s = y.iter().filter(|x| **x == 0).count();
            x_0s.cmp(&y_0s)
        })
        .unwrap()
        .to_vec();

    let ones = least_zeros.iter().filter(|n| **n == 1).count();
    let twos = least_zeros.iter().filter(|n| **n == 2).count();

    ones * twos
}

fn combine_layers(input: &Vec<usize>, area: usize) -> Vec<usize> {
    input
        .iter()
        .enumerate()
        .rev()
        .fold(vec![2; area], |mut acc, (i, n)| {
            acc[i % area] = match n {
                0 => 0,
                1 => 1,
                _ => acc[i % area],
            };

            acc
        })
}

fn show(input: &Vec<usize>, width: usize) -> String {
    input
        .iter()
        .map(|n| match n {
            1 => '#',
            _ => ' ',
        })
        .collect::<Vec<char>>()
        .chunks(width)
        .map(|line| format!("{}\n", line.iter().collect::<String>()))
        .collect()
}
