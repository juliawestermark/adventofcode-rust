use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, Write},
};

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

fn startup(day: i32) {
    println!();
    println!("{}--- 🎄 Day {:02} 🎄 ---{}", ANSI_BOLD, day, ANSI_RESET);
    println!();
}

fn ending() {
    println!();
    println!("{}--- 🎄🎄🎄🎄🎄🎄 ---{}", ANSI_BOLD, ANSI_RESET);
    println!();
}

fn parse_args() -> i32 {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let day: i32 = match args[1].parse() {
        Ok(day) => {
            if day < 1 || day > 25 {
                panic!("argument day is not in interval");
            }
            day
        },
        Err(_)=> panic!("argument day is invalid"),
    };
    day
}

const MODULE_TEMPLATE: &str = r###"use aoc2022::{
    lines_from_file,
    parse_args
};


fn part_one() {
    return 0
}

fn part_two() {
    return 0
}


fn main() {
    let filename = parse_args();
    let input = lines_from_file(filename);
    
    part1 = part_one();
    part2 = part_two();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
"###;

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn main() {
    let day = parse_args();
    let dday = format!("{:02}", day);

    let input_path = format!("src/inputs/input{}.txt", dday);
    let example_path = format!("src/examples/example{}.txt", dday);
    let module_path = format!("src/bin/day{}.rs", dday);

    startup(day);

    let file: Result<File, io::Error> = match safe_create_file(&module_path) {
        Ok(file) => Ok(file),
        Err(e) => {
            println!("Failed to create module file: {}", e);
            // process::exit(1);
            Err(e)
        }
    };
    
    if file.is_ok() {
        match file.expect("file").write_all(MODULE_TEMPLATE.as_bytes()) {
            Ok(_) => {
                println!("Created module file \"{}\"", &module_path);
            }
            Err(e) => {
                println!("Failed to write module contents: {}", e);
                // process::exit(1);
            }
        }
    }

    match safe_create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            println!("Failed to create input file: {}", e);
            // process::exit(1);
        }
    }

    match safe_create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            println!("Failed to create example file: {}", e);
            // process::exit(1);
        }
    }

    ending();
}


