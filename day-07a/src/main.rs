use itertools::Itertools;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

fn main() {
    let lines = io::stdin().lock().lines();

    let ops = [Op::Add, Op::Mul];

    let total: u64 = lines
        .map(|line| {
            let line = line.unwrap();
            let (expected_result, values) = line.split_once(": ").unwrap();
            let expected_result = expected_result.parse::<u64>().unwrap();
            let values: Vec<u64> = values
                .split(" ")
                .map(|v| v.parse::<u64>().unwrap())
                .collect();

            let possible_ops: Vec<Vec<Op>> = std::iter::repeat(ops)
                .take(values.len() - 1)
                .multi_cartesian_product()
                .map(|permutation| permutation.to_vec())
                .collect();

            for ops in possible_ops {
                let mut sum = values[0];
                for (index, op) in ops.iter().enumerate() {
                    sum = match op {
                        Op::Add => sum + values[index + 1],
                        Op::Mul => sum * values[index + 1],
                    }
                }
                if sum == expected_result {
                    return sum;
                }
            }

            0
        })
        .sum();

    println!("{}", total);
}
