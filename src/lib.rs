#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod error;

/// Input files
pub mod input;

use day1::group_max;
use day2::{CheatRpsMatch, RpsMatch};
use day3::{priority, Rucksack};
use day4::AssignmentPair;
use day5::{ActionsLines, Warehouse};
use day6::Stream;
use day7::{input_to_root, FsItem};
use error::Ooops;

/// Part A -> <https://adventofcode.com/2022/day/1>
pub fn total_of_calories_with_the_elf_with_the_most_calories(values: &str) -> usize {
    return group_max(values).fold(0, usize::max);
}

/// Part B -> <https://adventofcode.com/2022/day/1>
pub fn total_of_calories_for_the_top_three_elfs(values: &str) -> usize {
    let mut values = group_max(values).collect::<Vec<usize>>();
    values.sort();
    values.reverse();
    let values = values.iter();
    values.take(3).sum()
}

/// Part A -> <https://adventofcode.com/2022/day/2>
pub fn total_score_according_to_your_strategy_guide(values: &str) -> usize {
    values
        .trim()
        .split('\n')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.parse::<RpsMatch>().ok())
        .map(|v| v.play())
        .sum()
}

/// Part B -> <https://adventofcode.com/2022/day/2>
pub fn total_score_according_to_the_elfs_strategy_guide(values: &str) -> usize {
    values
        .trim()
        .split('\n')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.parse::<CheatRpsMatch>().ok())
        .map(|v| v.play())
        .sum()
}

/// Part A -> <https://adventofcode.com/2022/day/3>
pub fn the_sum_of_the_priorities_for_shared_item_types(values: &str) -> usize {
    values
        .trim()
        .lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.parse::<Rucksack>().ok())
        .map(|v| {
            v.shared
                .iter()
                .map(priority)
                .filter_map(|v| v.ok())
                .sum::<usize>()
        })
        .sum()
}

/// Part B -> <https://adventofcode.com/2022/day/3>
pub fn the_sum_of_the_priorities_for_shared_item_types_in_three_elfs_group(values: &str) -> usize {
    let mut iter = values
        .trim()
        .lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<Rucksack>())
        .filter_map(|v| v.ok());
    let mut results = vec![];
    while let (Some(one), Some(two), Some(three)) = (iter.next(), iter.next(), iter.next()) {
        results.push(one.intersection(vec![&two, &three]));
    }
    results
        .iter()
        .map(|v| v.iter().filter_map(|v| priority(v).ok()).sum::<usize>())
        .sum()
}

/// Part A -> <https://adventofcode.com/2022/day/4>
pub fn how_many_pairs_does_one_fully_contain_the_other(values: &str) -> usize {
    values
        .trim()
        .lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<AssignmentPair>())
        .filter_map(|v| v.ok())
        .filter(|v| v.a.fully_contains(&v.b) || v.b.fully_contains(&v.a))
        .count()
}

/// Part B -> <https://adventofcode.com/2022/day/4>
pub fn how_many_pairs_do_ranges_overlap(values: &str) -> usize {
    values
        .trim()
        .lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<AssignmentPair>())
        .filter_map(|v| v.ok())
        .filter(|v| v.a.overlaps(&v.b) || v.b.overlaps(&v.a))
        .count()
}

/// Part A -> <https://adventofcode.com/2022/day/5>
pub fn crates_on_top_of_each_stack(values: &str) -> Result<String, Ooops> {
    let mut warehouse: Warehouse = values.parse()?;
    let actions: ActionsLines = values.parse()?;
    for action in actions {
        warehouse.shuffle(&action);
    }
    Ok(warehouse.top_crates())
}

/// Part B -> <https://adventofcode.com/2022/day/5>
pub fn crates_on_top_of_each_stack_with_super_crane(values: &str) -> Result<String, Ooops> {
    let mut warehouse: Warehouse = values.parse()?;
    let actions: ActionsLines = values.parse()?;
    for action in actions {
        warehouse.shuffle_with_crane(&action, day5::CraneType::Super);
    }
    Ok(warehouse.top_crates())
}

/// Part A -> <https://adventofcode.com/2022/day/6>
pub fn start_of_packet_marker_position(values: &str) -> Option<usize> {
    let mut stream: Stream = values.into();
    stream.find_map(|w| w.marker_position())
}

/// Part B -> <https://adventofcode.com/2022/day/6>
pub fn start_of_message_marker_position(values: &str) -> Option<usize> {
    let mut stream: Stream = values.into();
    stream.window_size = 14;
    stream.find_map(|w| w.marker_position())
}

/// Part A -> <https://adventofcode.com/2022/day/7>
pub fn sum_of_the_total_sizes_of_directories_smaller_than(
    values: &str,
    size: usize,
) -> Result<usize, Ooops> {
    let root = input_to_root(values)?;
    let directories = root.ls_directories();
    let directories = directories
        .iter()
        .filter(|d| {
            let d = d.borrow();
            d.size() <= size
        })
        .map(|d| {
            let d = d.borrow();
            d.size()
        });
    Ok(directories.sum())
}

/// Part B -> <https://adventofcode.com/2022/day/7>
pub fn size_of_the_dir_to_be_deleted(
    values: &str,
    fs_size: usize,
    required_free_space: usize,
) -> Result<usize, Ooops> {
    let root = input_to_root(values)?;
    let root_size = root.size();
    let free_space = fs_size - root_size;
    let to_be_free = required_free_space - free_space;

    let directories = root.ls_directories();
    let directories = directories
        .iter()
        .filter(|d| {
            let d = d.borrow();
            d.size() >= to_be_free
        })
        .map(|d| {
            let d = d.borrow();
            d.size()
        });
    Ok(directories.min().unwrap_or(0))
}
