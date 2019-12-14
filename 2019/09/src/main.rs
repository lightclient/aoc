use common::load_input;
use vm::Vm;

fn main() {
    let code: Vec<i64> = load_input!(',', i64);

    let mut vm = Vm::new(code.clone(), vec![1]);
    let part1 = vm.run().unwrap();

    let mut vm = Vm::new(code, vec![2]);
    let part2 = vm.run().unwrap();

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
