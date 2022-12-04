use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, Write},
};

use aoc2022::{
    print_success,
    print_fail,
    print_startup,
    print_ending,
    print_advent_of_code,
    christmas_carol,
};

fn parse_args() -> i32 {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let day: i32 = match args[1].parse() {
        Ok(day) => {
            if day < 1 || day > 25 {
                -3
            }
            else {
                day
            }
        },
        Err(_)=> -4,
    };
    day
}

const MODULE_TEMPLATE: &str = r###"use aoc2022::{
    lines_from_file,
    parse_args
};


fn print_input(input: Vec<String>) {
    println!("Input:");
    println!("{:?}", input);
}

fn part_one(line_nbr: i32, line: String) -> String {
    format!("Line {}: {}", line_nbr, line)
}

fn part_two(the_best: bool) -> String {
    if the_best {
        return "You're the best!".to_string()
    }
    else {
        return "You are better than the best!".to_string()
    }
}

fn main() {
    let filename = parse_args();
    let input = lines_from_file(filename);

    print_input(input.clone());

    let mut part1 = Vec::<String>::new();
    
    let mut line_nbr = 1;
    for line in input {
        part1.push(part_one(line_nbr, line));
        line_nbr += 1;
    }

    let part2 = part_two(true);

    println!("Part 1: {:?}", part1); // :? is needed since part1 is a vector
    println!("Part 2: {:?}", part2);
}

"###;

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn main() {
    let day = parse_args();

    if day < 0 {
        print_advent_of_code();
        println!("{}", christmas_carol(day.abs()));
    }
    else {
        let dday = format!("{:02}", day);
        let input_path = format!("src/inputs/input{}.txt", dday);
        let example_path = format!("src/examples/example{}.txt", dday);
        let module_path = format!("src/bin/day{}.rs", dday);
        
        print_startup(day);

        let file: Result<File, io::Error> = match safe_create_file(&module_path) {
            Ok(file) => Ok(file),
            Err(e) => {
                let message = format!("Failed to create module file: {}", e);
                print_fail(message);
                // process::exit(1);
                Err(e)
            }
        };
        
        if file.is_ok() {
            match file.expect("file").write_all(MODULE_TEMPLATE.as_bytes()) {
                Ok(_) => {
                    let message = format!("Created module file \"{}\"", &module_path);
                    print_success(message);
                }
                Err(e) => {
                    let message = format!("Failed to write module contents: {}", e);
                    print_fail(message);
                    // process::exit(1);
                }
            }
        }

        match safe_create_file(&input_path) {
            Ok(_) => {
                let message = format!("Created empty input file \"{}\"", &input_path);
                print_success(message);
            }
            Err(e) => {
                let message = format!("Failed to create input file: {}", e);
                print_fail(message);
                // process::exit(1);
            }
        }

        match safe_create_file(&example_path) {
            Ok(_) => {
                let message = format!("Created empty example file \"{}\"", &example_path);
                print_success(message);
            }
            Err(e) => {
                let message = format!("Failed to create example file: {}", e);
                print_fail(message);
                // process::exit(1);
            }
        }
    }

    print_ending();
}


