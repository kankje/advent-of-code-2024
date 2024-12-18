use regex::Regex;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let re =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.*)")
            .unwrap();

    let (_, [reg_a, reg_b, reg_c, prog]) = re.captures(input.as_str()).unwrap().extract();

    let mut a = reg_a.parse::<u64>().unwrap();
    let mut b = reg_b.parse::<u64>().unwrap();
    let mut c = reg_c.parse::<u64>().unwrap();
    let instructions: Vec<u8> = prog
        .split(",")
        .map(|instruction| instruction.parse::<u8>().unwrap())
        .collect();

    let mut output = Vec::new();

    let mut pointer = 0;
    while pointer < instructions.len() - 1 {
        let instruction = instructions[pointer];
        let literal = instructions[pointer + 1] as u64;

        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Invalid operand"),
        };

        match instruction {
            0 => {
                a >>= combo;
            }
            1 => {
                b ^= literal;
            }
            2 => {
                b = combo % 8;
            }
            3 => {
                if a != 0 {
                    pointer = literal as usize;
                    continue;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                output.push((combo % 8).to_string());
            }
            6 => {
                b = a >> combo;
            }
            7 => {
                c = a >> combo;
            }
            _ => {}
        }

        pointer += 2;
    }

    println!("{}", output.join(","));
}
