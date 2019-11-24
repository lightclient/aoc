use std::io::{self, Read};

type Grid1 = [[bool; 1000]; 1000];
type Grid2 = [[u32; 1000]; 1000];

#[derive(Clone, Debug)]
enum Instruction {
    On(Section),
    Off(Section),
    Toggle(Section),
}

#[derive(Clone, Debug)]
struct Section(usize, usize, usize, usize);

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input.split('\n').filter(|l| l.len() > 0).collect();

    let mut grid1 = [[false; 1000]; 1000];
    let mut grid2 = [[0; 1000]; 1000];

    let config: Vec<Instruction> = lines.into_iter().map(parse_light_config).collect();

    let _ = config
        .iter()
        .map(|i| apply_instruction_part_1(i, &mut grid1))
        .collect::<()>();

    let _ = config
        .iter()
        .map(|i| apply_instruction_part_2(i, &mut grid2))
        .collect::<()>();

    let count = count_lights(&grid1);
    let sum = sum_intensity(&grid2);

    println!("There are {} lights on.", count);
    println!("There total intensity is {}.", sum);

    Ok(())
}

fn apply_instruction_part_1(instruction: &Instruction, grid: &mut Grid1) {
    let mut apply = |s: &Section, f: fn(bool) -> bool| {
        for c in s.0..=s.2 {
            for r in s.1..=s.3 {
                grid[c][r] = f(grid[c][r]);
            }
        }
    };

    match instruction {
        Instruction::On(s) => apply(s, |_| true),
        Instruction::Off(s) => apply(s, |_| false),
        Instruction::Toggle(s) => apply(s, |b: bool| !b),
    }
}

fn apply_instruction_part_2(instruction: &Instruction, grid: &mut Grid2) {
    let mut apply = |s: &Section, f: fn(u32) -> u32| {
        for c in s.0..=s.2 {
            for r in s.1..=s.3 {
                grid[c][r] = f(grid[c][r]);
            }
        }
    };

    match instruction {
        Instruction::On(s) => apply(s, |i| i + 1),
        Instruction::Off(s) => apply(s, |i| if i == 0 { 0 } else { i - 1 }),
        Instruction::Toggle(s) => apply(s, |i| i + 2),
    }
}

fn parse_light_config(config: &str) -> Instruction {
    let config: Vec<&str> = config.split_whitespace().collect();

    match config[0] {
        "turn" => match config[1] {
            "on" => Instruction::On(parse_section(config[2..].to_vec())),
            "off" => Instruction::Off(parse_section(config[2..].to_vec())),
            _ => unreachable!(),
        },
        "toggle" => Instruction::Toggle(parse_section(config[1..].to_vec())),
        _ => unreachable!(),
    }
}

fn parse_section(config: Vec<&str>) -> Section {
    let first: Vec<&str> = config[0].split(",").collect();
    let second: Vec<&str> = config[2].split(",").collect();

    Section(
        first[0].parse::<usize>().unwrap(),
        first[1].parse::<usize>().unwrap(),
        second[0].parse::<usize>().unwrap(),
        second[1].parse::<usize>().unwrap(),
    )
}

fn count_lights(grid: &Grid1) -> u32 {
    let mut count = 0;
    for c in 0..1000 {
        for r in 0..1000 {
            count += grid[c][r] as u32
        }
    }

    count
}

fn sum_intensity(grid: &Grid2) -> u32 {
    let mut count = 0;
    for c in 0..1000 {
        for r in 0..1000 {
            count += grid[c][r]
        }
    }

    count
}
