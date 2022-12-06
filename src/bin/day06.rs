use aoc2022::{
    lines_from_file,
    parse_args
};


fn get_marker(line: String, size: usize) -> usize {
    let vec: Vec<char>= line.chars().collect();
    let mut first = 0;
    let mut unique_len = 0;

    while unique_len < size {
        let mut current: Vec<&char> = vec.get(first..first+size).unwrap().iter().collect();
        current.sort();
        current.dedup();
        unique_len = current.len();
        first += 1;
    }
    first + size-1
}

fn main() {
    let filename = parse_args();
    let input = lines_from_file(filename);

    let mut part1 = Vec::<usize>::new();
    let mut part2 = Vec::<usize>::new();

    for line in input {
        let part1_one = get_marker(line.clone(), 4);
        let part2_two = get_marker(line.clone(), 14);
        part1.push(part1_one);
        part2.push(part2_two);
    }

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
