use regex::Regex;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let total: i64 = re
        .captures_iter(input.as_str())
        .map(|c| c.extract())
        .map(
            |(_, [btn_a_x, btn_a_y, btn_b_x, btn_b_y, prize_x, prize_y])| {
                let btn_a_x = btn_a_x.parse::<i64>().unwrap();
                let btn_a_y = btn_a_y.parse::<i64>().unwrap();
                let btn_b_x = btn_b_x.parse::<i64>().unwrap();
                let btn_b_y = btn_b_y.parse::<i64>().unwrap();
                let prize_x = prize_x.parse::<i64>().unwrap() + 10000000000000;
                let prize_y = prize_y.parse::<i64>().unwrap() + 10000000000000;

                let det = btn_a_x * btn_b_y - btn_b_x * btn_a_y;
                let det_x = prize_x * btn_b_y - btn_b_x * prize_y;
                let det_y = btn_a_x * prize_y - prize_x * btn_a_y;

                if det_x % det != 0 || det_y % det != 0 {
                    return 0;
                }

                let x = det_x / det;
                let y = det_y / det;

                x * 3 + y
            },
        )
        .sum();

    println!("{}", total);
}
