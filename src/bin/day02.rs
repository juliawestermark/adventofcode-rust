use aoc2022::{
    lines_from_file,
    parse_args
};
use std::io;


enum Hand {
    Rock = 1, // Rock
    Paper = 2, // Paper
    Scissor = 3, // Scissor
}

fn get_hand2(input: &str) -> i32 {
    match input {
        "A"  => 1,
        "B"  => 2,
        "C"  => 3,
        "X"  => 1,
        "Y"  => 2,
        "Z"  => 3,
        _    => {
            println!("Wrong");
            0
        }
    }
}

fn get_points(input: &str) -> i32 {
    match input {
        "X"  => 0,
        "Y"  => 3,
        "Z"  => 6,
        _    => {
            println!("Wrong");
            0
        }
    }
}

fn compare(opponent: i32, you: i32) -> i32 {
    if opponent == you {
        return 3; // draw
    }
    else if opponent == Hand::Scissor as i32 {
        if you == Hand::Rock as i32 {
            return 6; // win
        }
        else {
            return 0; // lost
        }
    }
    else if you == Hand::Scissor as i32 {
        if opponent == Hand::Rock as i32 {
            return 0; // lost
        }
        else {
            return 6; // win
        }
    }
    else if opponent < you {
        return 6; // win
    }
    else {
        return 0; // lost
    }
}

fn get_hand_from_results(opponent: i32, you: i32) -> i32 {
    if you == 0 {
        if opponent == 1 {
            return 3
        }
        else {
            return opponent - 1
        }
    }
    else if you == 3 {
        return opponent
    }
    else if you == 6 {
        if opponent == 3 {
            return 1
        }
        else {
            return opponent + 1
        }
    }
    else {
        println!("Something is wrong");
        return 0
    }
}

fn part_one(vec: Vec<&str>, verbose: i32) -> i32 {
    let opponent = get_hand2(vec[0]);
    let you = get_hand2(vec[1]);
    let comp = compare(opponent,you);
    if verbose > 0 {
        println!("hand {} + comp {} = totalt {}", you, comp, comp+you);
    }
    comp + you
}

fn part_two(vec: Vec<&str>, verbose: i32) -> i32 {
    let opponent = get_hand2(vec[0]);
    let comp = get_points(vec[1]);
    let you = get_hand_from_results(opponent, comp);
    if verbose > 0 {
        println!("hand {} + comp {} = totalt {}", you, comp, comp+you);
    }
    comp + you
}


fn main() {
    let filename = parse_args();
    let input = lines_from_file(filename);
    let verbose = 0;

    let mut points_part1 = 0;
    let mut points_part2 = 0;
    for line in input {
        let vec: Vec<&str> = line.split(" ").collect();
        points_part1 += part_one(vec.clone(), verbose);
        points_part2 += part_two(vec.clone(), verbose);
    }

    println!("Part 1: {}", points_part1);
    println!("Part 2: {}", points_part2);
}
