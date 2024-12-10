use itertools::Itertools;
use std::io;
use std::iter::repeat;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let mut disk_map: Vec<(u64, u64, u64)> = input
        .chars()
        .tuples()
        .enumerate()
        .map(|(file_id, (file_block_count, free_block_count))| {
            (
                file_id as u64,
                file_block_count.to_string().parse::<u64>().unwrap_or(0),
                free_block_count.to_string().parse::<u64>().unwrap_or(0),
            )
        })
        .collect();

    let mut end_index = disk_map.len() - 1;
    while end_index > 0 {
        for start_index in 0..end_index {
            if disk_map[start_index].2 < disk_map[end_index].1 {
                continue;
            }

            let mut moved_file = disk_map[end_index];
            moved_file.2 = disk_map[start_index].2 - disk_map[end_index].1;
            disk_map[end_index].2 += disk_map[end_index].1;
            disk_map[end_index].1 = 0;
            disk_map[start_index].2 = 0;
            disk_map.insert(start_index + 1, moved_file);
            end_index += 1;
            break;
        }

        end_index -= 1;
    }

    let blocks: Vec<Option<u64>> = disk_map
        .iter()
        .flat_map(|(file_id, file_block_count, free_block_count)| {
            repeat(Some(*file_id))
                .take(*file_block_count as usize)
                .chain(repeat(None).take(*free_block_count as usize))
                .collect::<Vec<Option<u64>>>()
        })
        .collect();

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
