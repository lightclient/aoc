use common::load_input;
use vm::Vm;

fn main() {
    let code: Vec<i64> = load_input!(',', i64);

    let mut vm = Vm::new(&code);
    let part1 = vm.run(vec![1]).unwrap();

    let mut vm = Vm::new(&code);
    let part2 = vm.run(vec![2]).unwrap();

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
