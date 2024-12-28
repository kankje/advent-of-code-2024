use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (_, gates) = input.split_once("\n\n").unwrap();
    let gates: Vec<_> = gates
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<_>>();
            (parts[1], parts[0], parts[2], parts[4])
        })
        .collect();

    let mut swapped = Vec::new();

    // https://en.wikipedia.org/wiki/Adder_(electronics)
    for &(op, left, right, out) in &gates {
        // Ignore first/last gate
        if ((left == "x00" && right == "y00") || (left == "y00" && right == "x00")) || out == "z45"
        {
            continue;
        }

        let next_gates: Vec<_> = gates
            .iter()
            .filter(|(_, left, right, _)| left == &out || right == &out)
            .collect();

        match op {
            "AND" => {
                // AND -> OR
                if !next_gates.iter().any(|(op, _, _, _)| op == &"OR") {
                    swapped.push(out);
                }
            }

            "OR" => {
                // OR -> AND & XOR
                if !next_gates.iter().any(|(op, _, _, _)| op == &"AND")
                    || !next_gates.iter().any(|(op, _, _, _)| op == &"XOR")
                {
                    swapped.push(out);
                }
            }

            "XOR" => {
                let has_input = left.starts_with('x')
                    || left.starts_with('y')
                    || right.starts_with('x')
                    || right.starts_with('y');

                // Can only output from non-input wires
                if !has_input && out.starts_with('z') {
                    continue;
                }

                // Has input wire, must be XOR -> XOR
                if has_input && next_gates.iter().any(|(op, _, _, _)| op == &"XOR") {
                    continue;
                }

                swapped.push(out);
            }
            _ => panic!("Invalid operation"),
        }
    }

    swapped.sort();

    println!("{}", swapped.join(","));
}
