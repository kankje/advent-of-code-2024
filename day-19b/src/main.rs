use std::collections::HashMap;
use std::io;

fn find_combinations(design: &str, patterns: &Vec<&str>, counts: &mut HashMap<String, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(count) = counts.get(design) {
        return *count;
    }

    let mut count = 0;

    for pattern in patterns {
        let Some(design_without_pattern) = design.strip_prefix(pattern) else {
            continue;
        };

        count += find_combinations(design_without_pattern, patterns, counts);
    }

    counts.insert(design.to_string(), count);

    count
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let [patterns, _, designs @ ..] = lines.as_slice() else {
        panic!("Invalid input");
    };
    let patterns: Vec<&str> = patterns.split(", ").collect();

    let count: u64 = designs
        .iter()
        .map(|design| find_combinations(design, &patterns, &mut HashMap::new()))
        .sum();

    println!("{}", count);
}
