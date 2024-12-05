use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let (rules_raw, updates_raw) = input.split_once("\n\n").unwrap();
    let rules = rules_raw
        .lines()
        .filter_map(|line| line.split_once("|"))
        .fold(HashMap::new(), |mut acc, rule| {
            let v: &mut Vec<i32> = acc.entry(rule.1.parse::<i32>().unwrap()).or_default();
            v.push(rule.0.parse::<i32>().unwrap());
            acc
        });
    let updates: Vec<Vec<i32>> = updates_raw
        .lines()
        .map(|line| {
            line.split(",")
                .map(|page| page.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let total: i32 = updates
        .iter()
        .map(|update_pages| {
            let mut sorted_update_pages = update_pages.clone();

            sorted_update_pages.sort_by(|a, b| {
                if let Some(must_be_befores) = rules.get(a) {
                    for must_be_before in must_be_befores {
                        if b == must_be_before {
                            return Ordering::Greater;
                        }
                    }
                }

                if let Some(must_be_befores) = rules.get(b) {
                    for must_be_before in must_be_befores {
                        if a == must_be_before {
                            return Ordering::Less;
                        }
                    }
                }

                Ordering::Equal
            });

            if sorted_update_pages
                .iter()
                .enumerate()
                .any(|(index, page)| update_pages[index] != *page)
            {
                sorted_update_pages[sorted_update_pages.len() / 2]
            } else {
                0
            }
        })
        .sum();

    println!("{}", total);
}
