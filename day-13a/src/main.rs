use regex::Regex;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let total: i32 = re
        .captures_iter(input.as_str())
        .map(|c| c.extract())
        .map(
            |(_, [btn_a_x, btn_a_y, btn_b_x, btn_b_y, prize_x, prize_y])| {
                let btn_a_x = btn_a_x.parse::<i32>().unwrap();
                let btn_a_y = btn_a_y.parse::<i32>().unwrap();
                let btn_b_x = btn_b_x.parse::<i32>().unwrap();
                let btn_b_y = btn_b_y.parse::<i32>().unwrap();
                let prize_x = prize_x.parse::<i32>().unwrap();
                let prize_y = prize_y.parse::<i32>().unwrap();
                let mut presses_for_prizes: Option<(i32, i32, i32)> = None;
                for a_presses in 0..100 {
                    let a_x = btn_a_x * a_presses;
                    let a_y = btn_a_y * a_presses;
                    for b_presses in 0..100 {
                        let b_x = btn_b_x * b_presses;
                        let b_y = btn_b_y * b_presses;
                        if (a_x + b_x) == prize_x && (a_y + b_y) == prize_y {
                            let new = (a_presses, b_presses, a_presses * 3 + b_presses);
                            presses_for_prizes = match presses_for_prizes {
                                Some(prev) => {
                                    if prev.2 > new.2 {
                                        Some(new)
                                    } else {
                                        Some(prev)
                                    }
                                }
                                None => Some(new),
                            }
                        }
                    }
                }
                match presses_for_prizes {
                    Some((_, _, tokens)) => tokens,
                    None => 0,
                }
            },
        )
        .sum();

    println!("{}", total);
}
