# Advent of Code 2022

```rust
use aoc::*;

// Day 1
assert_eq!(
    total_of_calories_with_the_elf_with_the_most_calories(input::DAY1),
    69693
);
assert_eq!(
    total_of_calories_for_the_top_three_elfs(input::DAY1),
    200945
);

// Day 2
assert_eq!(
    total_score_according_to_your_strategy_guide(input::DAY2),
    14827
);
assert_eq!(
    total_score_according_to_the_elfs_strategy_guide(input::DAY2),
    13889
);

// Day 3
assert_eq!(
    the_sum_of_the_priorities_for_shared_item_types(input::DAY3),
    8153
);
assert_eq!(
    the_sum_of_the_priorities_for_shared_item_types_in_three_elfs_group(input::DAY3),
    2342
);

// Day 4
assert_eq!(
    how_many_pairs_does_one_fully_contain_the_other(input::DAY4),
    584
);
assert_eq!(how_many_pairs_do_ranges_overlap(input::DAY4), 933);

// Day 5
assert_eq!(
    crates_on_top_of_each_stack(input::DAY5).unwrap(),
    "FWSHSPJWM".to_string()
);
assert_eq!(
    crates_on_top_of_each_stack_with_super_crane(input::DAY5).unwrap(),
    "PWPWHGFZS".to_string()
);

// Day 6
assert_eq!(
    start_of_packet_marker_position(input::DAY6).unwrap(),
    1578
);
assert_eq!(
    start_of_message_marker_position(input::DAY6).unwrap(),
    2178
);

// Day 7
assert_eq!(
    sum_of_the_total_sizes_of_directories_smaller_than(input::DAY7, 100000).unwrap(),
    1908462
);
assert_eq!(
    size_of_the_dir_to_be_deleted(input::DAY7, 70000000, 30000000).unwrap(),
    3979145
);

// Day 8
assert_eq!(
    trees_visible_from_outside_the_grid(input::DAY8),
    1676
);
assert_eq!(
    highest_scenic_score_possible(input::DAY8),
    313200
);
```
