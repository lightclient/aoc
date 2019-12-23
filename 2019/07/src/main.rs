use common::load_input;
use vm::{Status, Vm};

fn main() {
    let code: Vec<i64> = load_input!(',', i64);

    let permutations1 = heaps_alg(vec![0, 1, 2, 3, 4]);
    let permutations2 = heaps_alg(vec![5, 6, 7, 8, 9]);

    let mut part1 = vec![];
    let mut part2 = vec![];

    for p in permutations1 {
        part1.push(run(&code, p));
    }

    for p in permutations2 {
        part2.push(run(&code, p));
    }

    println!("Part 1 output: {}", part1.iter().max().unwrap());
    println!("Part 2 output: {}", part2.iter().max().unwrap());
}

fn heaps_alg(v: Vec<i64>) -> Vec<Vec<i64>> {
    fn f(len: usize, v: &mut Vec<i64>, ret: &mut Vec<Vec<i64>>) {
        if len == 0 || len == 1 {
            ret.push(v.to_owned());
            return;
        }

        for i in 0..len - 1 {
            f(len - 1, v, ret);
            let j = if len % 2 == 0 { i } else { 0 };
            v.swap(j, len - 1);
        }

        f(len - 1, v, ret);
    }

    let mut ret = vec![];
    let mut v = v.clone();

    f(v.len(), &mut v, &mut ret);

    ret
}

fn run(code: &[i64], p: Vec<i64>) -> i64 {
    let simulate_thruster = |vm: &mut Vm, input: i64, last: i64| -> i64 {
        loop {
            match vm.step() {
                Ok(Status::Output(o)) => break o,
                Ok(Status::AwaitingInput) => vm.insert_input(input),
                Ok(Status::Halted) => break last,
                _ => continue,
            }
        }
    };

    let mut vm = vec![Vm::new(code); 5];
    let mut ret = vec![0; 5];
    let mut i = 0;

    for (i, v) in vm.iter_mut().enumerate() {
        v.insert_input(p[i]);
    }

    loop {
        ret[i % 5] = simulate_thruster(&mut vm[i % 5], ret[(4 + i) % 5], ret[i % 5]);

        if vm[i % 5].halted() {
            break;
        }

        i += 1;
    }

    ret[4]
}
