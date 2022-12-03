use aoc2022::{
    lines_from_file,
    parse_args
};

fn get_char_value(ch: char) -> u32{
    let cha = ch as u32;
    let points;
    if cha > 90 {
        points = cha - 96;
    }
    else {
        points = cha - 64 + 26;
    }
    return points
}

fn part_one(input: Vec<String>) -> u32 {
    let mut priority_sum = 0;

    for line in input {
        let lenline = line.len();
        let (first, second) = line.split_at(lenline/2);
        let firstc = first.chars().collect::<Vec<char>>();
        let mut in_both = Vec::new();
        for ch in firstc {
            if second.contains(ch) {
                in_both.push(get_char_value(ch));
            }
        }
        in_both.dedup();
        let priority = in_both.iter().sum::<u32>();

        priority_sum += priority;
    }

    priority_sum
}

fn part_two(input: Vec<String>) -> u32 {
    let mut priority_sum = 0;

    let mut i = 0;
    while i < input.len() {
        let mut in_all = Vec::new();
        let group = input.get(i..i+3).unwrap();
        let first = group.get(0).unwrap().chars().collect::<Vec<char>>();
        for ch in first {
            if group.get(1).unwrap().contains(ch) && group.get(2).unwrap().contains(ch) {
                in_all.push(get_char_value(ch));
            }
        }
        in_all.dedup();
        let priority = in_all.iter().sum::<u32>();

        priority_sum += priority;
        i += 3;
    }
    priority_sum
}


fn main() {
    let filename = parse_args();
    let input = lines_from_file(filename);

    let part1 = part_one(input.clone());
    let part2 = part_two(input.clone());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
