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
            let is_valid = update_pages
                .iter()
                .enumerate()
                .fold(true, |is_valid, (index, page)| {
                    if !is_valid {
                        return false;
                    }

                    let Some(must_be_befores) = rules.get(page) else {
                        return true;
                    };

                    for must_be_before in must_be_befores {
                        if !update_pages.contains(must_be_before) {
                            continue;
                        }

                        if !update_pages[0..index].contains(must_be_before) {
                            return false;
                        }
                    }

                    true
                });

            if is_valid {
                update_pages[update_pages.len() / 2]
            } else {
                0
            }
        })
        .sum();

    println!("{}", total);
}
