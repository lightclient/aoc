use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut ops: Vec<usize> = input
        .split(",")
        .map(|n| n.replace(&['\n'][..], ""))
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    // specific initialization
    ops[1] = 12;
    ops[2] = 2;

    let initial_mem = process_ops(&ops);
    let (noun, verb) = find_inputs(&ops);

    println!("mem[0]: {}", initial_mem[0]);
    println!("100 * noun + verb = {}", 100 * noun + verb);

    Ok(())
}

fn find_inputs(ops: &[usize]) -> (usize, usize) {
    let mut ret;
    let mut mem;

    for i in 0..ops.len() {
        for j in 0..ops.len() {
            mem = ops.clone().to_vec();
            mem[1] = i;
            mem[2] = j;
            mem = process_ops(&mem);
            ret = mem[0];

            if ret == 19690720 {
                return (i, j);
            }
        }
    }

    panic!("invalid program");
}

fn process_ops(ops: &[usize]) -> Vec<usize> {
    let mut i = 0;
    let mut ops = ops.to_vec();

    while i < ops.len() {
        match ops[i] {
            1 | 2 => {
                if let [x, y, p] = ops[i + 1..=i + 3] {
                    ops[p] = op(ops[i], ops[x], ops[y]);
                    i += 4;
                }
            }
            99 => return ops.to_vec(),
            _ => panic!("invalid sequence"),
        }
    }

    panic!("invalid program");
}

fn op(n: usize, x: usize, y: usize) -> usize {
    match n {
        1 => x + y,
        2 => x * y,
        _ => unreachable!(),
    }
}
