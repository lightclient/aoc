use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Clone, Debug)]
enum Value {
    Constant(u16),
    Wire(String),
}

#[derive(Clone, Debug)]
enum Op {
    Assignment(Value),
    Not(Value),
    And(Value, Value),
    Or(Value, Value),
    Shl(Value, Value),
    Shr(Value, Value),
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .collect::<Vec<&str>>();

    let mut map = read_ops(&lines);

    let results1 = process_ops(&mut map.clone());

    map.insert(
        "b".to_string(),
        Op::Assignment(Value::Constant(results1["a"])),
    );

    let results2 = process_ops(&mut map);

    println!("Wire `a` is equal to {} in part 1.", results1["a"]);
    println!("Wire `a` is equal to {} in part 2.", results2["a"]);

    Ok(())
}

fn process_ops(map: &mut HashMap<String, Op>) -> HashMap<String, u16> {
    let mut ret = HashMap::<String, u16>::new();

    while map.len() > 0 {
        for (k, v) in map.clone() {
            match v {
                Op::Assignment(v) => uniary_op(&mut ret, map, k, v, |a| a),
                Op::Not(v) => uniary_op(&mut ret, map, k, v, |a| !a),
                Op::And(a, b) => binary_op(&mut ret, map, k, a, b, |a, b| a & b),
                Op::Or(a, b) => binary_op(&mut ret, map, k, a, b, |a, b| a | b),
                Op::Shl(a, b) => binary_op(&mut ret, map, k, a, b, |a, b| a << b),
                Op::Shr(a, b) => binary_op(&mut ret, map, k, a, b, |a, b| a >> b),
            }
        }
    }

    ret
}

fn uniary_op(
    ret: &mut HashMap<String, u16>,
    map: &mut HashMap<String, Op>,
    k: String,
    v: Value,
    uniary_op: fn(u16) -> u16,
) {
    match v {
        Value::Constant(n) => {
            ret.entry(k.clone()).or_insert(uniary_op(n));
            map.remove(&k);
        }
        Value::Wire(w) => match ret.clone().get(&w) {
            Some(a) => {
                ret.insert(k.clone(), uniary_op(*a));
                map.remove(&k);
            }
            None => (),
        },
    }
}

fn binary_op(
    ret: &mut HashMap<String, u16>,
    map: &mut HashMap<String, Op>,
    k: String,
    a: Value,
    b: Value,
    binary_op: fn(u16, u16) -> u16,
) {
    let left = match a {
        Value::Constant(n) => Some(n),
        Value::Wire(w) => ret.get(&w).map(|v| *v),
    };

    let right = match b {
        Value::Constant(n) => Some(n),
        Value::Wire(w) => ret.get(&w).map(|v| *v),
    };

    match (left, right) {
        (Some(x), Some(y)) => {
            ret.insert(k.clone(), binary_op(x, y));
            map.remove(&k);
        }
        _ => (),
    }
}

fn read_ops(lines: &Vec<&str>) -> HashMap<String, Op> {
    let mut map = HashMap::<String, Op>::new();

    for line in lines {
        let line: Vec<&str> = line.split_whitespace().collect();

        match line.len() {
            // assignment
            3 => map.insert(
                line[2].to_string(),
                match line[0].parse::<u16>() {
                    Ok(n) => Op::Assignment(Value::Constant(n)),
                    Err(_) => Op::Assignment(Value::Wire(line[0].to_string())),
                },
            ),

            // uniary op
            4 => match line[1].parse::<u16>() {
                Ok(_) => panic!("invalid NOT"),
                Err(_) => map.insert(
                    line[3].to_string(),
                    Op::Not(Value::Wire(line[1].to_string())),
                ),
            },

            // binary op
            5 => match line[1] {
                "AND" => {
                    let left: Value = line[0].into();
                    let right: Value = line[2].into();
                    map.insert(line[4].to_string(), Op::And(left, right))
                }
                "OR" => {
                    let left: Value = line[0].into();
                    let right: Value = line[2].into();
                    map.insert(line[4].to_string(), Op::Or(left, right))
                }
                "LSHIFT" => {
                    let left: Value = line[0].into();
                    let right = match line[2].parse::<u16>() {
                        Ok(n) => Value::Constant(n),
                        Err(_) => panic!("invalid shl"),
                    };
                    map.insert(line[4].to_string(), Op::Shl(left, right))
                }
                "RSHIFT" => {
                    let left: Value = line[0].into();
                    let right = match line[2].parse::<u16>() {
                        Ok(n) => Value::Constant(n),
                        Err(_) => panic!("invalid shr"),
                    };
                    map.insert(line[4].to_string(), Op::Shr(left, right))
                }
                _ => panic!("invalid op"),
            },
            _ => panic!("invalid op"),
        };
    }

    map
}

impl std::convert::From<&str> for Value {
    fn from(s: &str) -> Self {
        match s.parse::<u16>() {
            Ok(n) => Value::Constant(n),
            Err(_) => Value::Wire(s.to_string()),
        }
    }
}
