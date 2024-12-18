use regex::Regex;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let re =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.*)")
            .unwrap();

    let (_, [_, _, _, prog]) = re.captures(input.as_str()).unwrap().extract();

    let instructions: Vec<u8> = prog
        .split(",")
        .map(|instruction| instruction.parse::<u8>().unwrap())
        .collect();

    let mut potential_next_as = vec![0u64];
    for instruction in instructions.iter().rev() {
        let l = potential_next_as.clone();
        potential_next_as.clear();
        for last_bits in 0..=7 {
            for a in &l {
                let mut a: u64 = (a << 3) + last_bits as u64;
                let mut b: u64 = a % 8;
                b ^= 5;
                let c: u64 = a >> b;
                b ^= 6;
                b ^= c;
                if (b % 8) as u8 == *instruction {
                    potential_next_as.push(a);
                }
            }
        }
        if potential_next_as.is_empty() {
            panic!("No A candidates found");
        }
    }

    println!("{}", potential_next_as.iter().min().unwrap());
}
