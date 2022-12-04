use aoc2022::{lines_from_file, parse_args};

use std::env;


fn count_calories(input: Vec<String>) -> Vec<i32> {
    let mut calories = Vec::new();
    let mut current = 0;

    for line in input {
        if line.is_empty() {
            calories.push(current);
            current = 0;
        }
        else {
            let line_val = line.parse::<i32>().unwrap();
            current += line_val;
        }
    }
    if current != 0 {
        calories.push(current);
    }

    calories.sort_by(|a, b| b.cmp(a));
    return calories
}


fn main() {
    let filename = parse_args();
    let input = lines_from_file(filename);

    let calories = count_calories(input);

    let (high_part1, _) = calories.split_at(1);
    let (high_part2, _) = calories.split_at(3);
    let highest_part1: i32 = high_part1.iter().sum();
    let highest_part2: i32 = high_part2.iter().sum();

    println!("Part 1: {}", highest_part1);
    println!("Part 2: {}", highest_part2);
}
