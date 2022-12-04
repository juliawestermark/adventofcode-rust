use aoc2022::{
    lines_from_file,
    parse_args
};

fn get_sections(first: i32, second: i32) -> Vec<i32> {
    (first..second+1).collect()
}

fn part_one(sections1: Vec<i32>, sections2: Vec<i32>) -> i32 {
    if sections1.len() < sections2.len() {
        sections1.iter().all(|s| sections2.contains(s)) as i32
    }
    else {
        sections2.iter().all(|s| sections1.contains(s)) as i32
    }
}

fn part_two(short: Vec<i32>, long: Vec<i32>) -> i32 {
    short.iter().any(|s| long.contains(s)) as i32
}


fn main() {
    let filename = parse_args();
    let input = lines_from_file(filename);

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    for line in input {
        let vec: Vec<i32> = line.split(['-',',']).map(|s| s.parse().unwrap()).collect();
        let elf1 = get_sections(vec[0], vec[1]);
        let elf2 = get_sections(vec[2], vec[3]);
        sum_part1 += part_one(elf1.clone(), elf2.clone());
        sum_part2 += part_two(elf1.clone(), elf2.clone());
    }

    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
}
