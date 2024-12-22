use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn next_secret(secret: u64) -> u64 {
    let mut result = secret;
    result ^= result * 64;
    result %= 16777216;
    result ^= result / 32;
    result %= 16777216;
    result ^= result * 2048;
    result %= 16777216;
    result
}

fn main() {
    let lines = io::stdin().lock().lines();
    let initial_secrets: Vec<u64> = lines
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();

    let mut all_price_change_profits = Vec::new();

    for initial_secret in &initial_secrets {
        let (_, bananas) =
            (0..2000).fold((*initial_secret, Vec::new()), |(secret, mut bananas), _| {
                let secret = next_secret(secret);
                bananas.push(secret % 10);
                (secret, bananas)
            });

        let changes: Vec<i64> = bananas
            .windows(2)
            .map(|w| w[1] as i64 - w[0] as i64)
            .collect();

        let mut price_change_profits = HashMap::new();

        for (i, seq) in changes.windows(4).enumerate() {
            price_change_profits
                .entry((seq[0], seq[1], seq[2], seq[3]))
                .or_insert(bananas[i + 4]);
        }

        all_price_change_profits.push(price_change_profits);
    }

    let mut total = HashMap::new();

    for price_change_profit in all_price_change_profits {
        for (price_change, sold) in price_change_profit {
            let counter = total.entry(price_change).or_insert(0);
            *counter += sold;
        }
    }

    let (_, highest_sold) = total.iter().max_by_key(|(_, amount)| *amount).unwrap();

    println!("{}", highest_sold);
}
