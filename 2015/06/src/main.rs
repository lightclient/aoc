use common::load_input;

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

fn main() {
    let config: Vec<Instruction> = load_input!("\n", self::parse_light_config);

    let mut grid1 = [[false; 1000]; 1000];
    let mut grid2 = [[0; 1000]; 1000];

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

fn parse_light_config(config: String) -> Instruction {
    let config: Vec<String> = config.split_whitespace().map(|s| s.to_string()).collect();

    match config[0].as_ref() {
        "turn" => match config[1].as_ref() {
            "on" => Instruction::On(parse_section(config[2..].to_vec())),
            "off" => Instruction::Off(parse_section(config[2..].to_vec())),
            _ => unreachable!(),
        },
        "toggle" => Instruction::Toggle(parse_section(config[1..].to_vec())),
        _ => unreachable!(),
    }
}

fn parse_section(config: Vec<String>) -> Section {
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
