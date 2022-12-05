use aoc2022::{
    lines_from_file,
    parse_args
};


fn clean_containers(line: &str) -> Vec<String> {
    let mut part1 = Vec::<String>::new();
    let matcher: &[_] = &['[', ']', ' '];
    for chunk in line.as_bytes().chunks(4).map(std::str::from_utf8).collect::<Result<Vec<&str>, _>>().unwrap() {
        let ch = chunk.trim_matches(matcher);
        part1.push(ch.to_string());
    }
    part1
}

fn clean_instructions(line: &str) -> Vec<usize> {
    let nbrcol: Vec<char> = line.chars().collect();
    let vec: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
    let mut nn = Vec::<usize>::new();
    for i in vec![1,3,5] {
        let temp: usize = vec[i].parse().unwrap();
        nn.push(temp)
    }
    nn
}


fn main() {
    let filename = parse_args();
    let mut input = lines_from_file(filename);

    let index = input.iter().position(|x| x.is_empty()).unwrap();
    let (first, second) = input.split_at_mut(index+1);
    first.reverse();
    let vec_len: usize = first.get(1).expect("get string").trim().split(" ").last().unwrap().parse().unwrap();
    let first_fix = first.get(2..first.len()).unwrap();
    
    let mut vec: Vec::<Vec<String>> = vec![Vec::new();vec_len];
    let mut vec2: Vec::<Vec<String>> = vec![Vec::new();vec_len];
    
    for line in first_fix {
        let containers = clean_containers(line);
        for i in (0..vec_len) {
            if ! containers[i].is_empty() {
                vec[i].push(containers[i].to_string());
                vec2[i].push(containers[i].to_string());
            }
        }
    }
    
    for line in second {
        let instructions = clean_instructions(line);
        
        for i in (1..instructions[0]+1) {
            let element = vec[instructions[1]-1].pop();
            vec[instructions[2]-1].push(element.unwrap());
        }
        let v_len = vec2[instructions[1]-1].len();
        let mut u2 = vec2[instructions[1]-1].split_off(v_len-instructions[0]);
        vec2[instructions[2]-1].append(&mut u2);
    }

    let mut part1 = Vec::<String>::new();
    let mut part2 = Vec::<String>::new();

    for i in (0..vec.len()) {
        part1.push(vec[i].pop().unwrap());
        part2.push(vec2[i].pop().unwrap());
    }
    
    println!("Part 1: {}", part1.join(""));
    println!("Part 2: {}", part2.join(""));
}

