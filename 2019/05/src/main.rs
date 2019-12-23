use common::load_input;
use vm::Vm;

fn main() {
    let code: Vec<i64> = load_input!(',', i64);

    let mut vm = Vm::new(&code);

    let output1 = vm.run(vec![1]).unwrap();
    let output2 = vm.run(vec![5]).unwrap();

    println!("Part 1 output: {}", output1);
    println!("Part 2 output: {}", output2);
}
