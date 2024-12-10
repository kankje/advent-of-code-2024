use itertools::Itertools;
use std::io;
use std::iter::repeat;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let mut blocks: Vec<Option<u64>> = input
        .chars()
        .tuples()
        .enumerate()
        .flat_map(|(file_id, (file_block_count, free_block_count))| {
            repeat(Some(file_id as u64))
                .take(file_block_count.to_string().parse::<usize>().unwrap_or(0))
                .chain(
                    repeat(None).take(free_block_count.to_string().parse::<usize>().unwrap_or(0)),
                )
                .collect::<Vec<Option<u64>>>()
        })
        .collect();

    let mut start_index = 0;
    let mut end_index = blocks.len() - 1;
    while start_index < blocks.len() && end_index > start_index {
        if blocks[start_index].is_some() {
            start_index += 1;
            continue;
        }

        while end_index > start_index {
            if blocks[end_index].is_some() {
                blocks.swap(start_index, end_index);
                end_index -= 1;
                break;
            }

            end_index -= 1;
        }

        start_index += 1;
    }

    let total: u64 = blocks
        .iter()
        .enumerate()
        .map(|(index, block)| match block {
            Some(block) => block * index as u64,
            None => 0,
        })
        .sum();

    println!("{}", total);
}
