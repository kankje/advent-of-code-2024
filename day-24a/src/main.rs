use std::collections::HashMap;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (initial_values, gates) = input.split_once("\n\n").unwrap();
    let mut values = initial_values
        .lines()
        .fold(HashMap::new(), |mut values, line| {
            let (gate, value) = line.split_once(": ").unwrap();
            values.insert(gate, value.parse::<u64>().unwrap());
            values
        });
    let mut gates: Vec<_> = gates
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<_>>();
            (parts[1], parts[0], parts[2], parts[4])
        })
        .collect();

    while !gates.is_empty() {
        for (index, operation) in gates.iter().enumerate() {
            let (op, gate_a, gate_b, gate_c) = operation;
            let (Some(value_a), Some(value_b)) = (values.get(gate_a), values.get(gate_b)) else {
                continue;
            };

            let result = match *op {
                "AND" => value_a & value_b,
                "OR" => value_a | value_b,
                "XOR" => value_a ^ value_b,
                _ => panic!("Invalid operation"),
            };

            values.insert(gate_c, result);
            gates.remove(index);
            break;
        }
    }

    let mut zs: Vec<_> = values
        .iter()
        .filter(|(gate, _)| gate.starts_with("z"))
        .collect();
    zs.sort_by_key(|(gate, _)| *gate);
    let bits: Vec<_> = zs
        .iter()
        .rev()
        .map(|(_, value)| value.to_string())
        .collect();
    let result = u64::from_str_radix(bits.join("").as_str(), 2).unwrap();

    println!("{}", result);
}
