use std::env;
use std::process::Command;

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

struct Config {
    day: i32,
    input_filename: String,
}

// fn help() {
//     println!("usage:
// match_args <string>
//     Check whether given string is the answer.
// match_args {{increase|decrease}} <integer>
//     Increase or decrease given integer by one.");
// }

fn parse_config(args: &[String]) -> Config {

    let input_filename;

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

    if args.len() < 3 {
        input_filename = format!("src/inputs/input{:02}.txt", day).to_string();
    }
    else {
        match args[2].as_str() {
            "result" => {
                input_filename = format!("src/inputs/input{:02}.txt", day).to_string();
            },
            "test" => {
                input_filename = format!("src/examples/example{:02}.txt", day).to_string();
            },
            _ => {
                input_filename = format!("src/inputs/input{:02}.txt", day).to_string();
            },
        }
    }

    Config {day, input_filename}

}


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    let input_filename = config.input_filename;
    let day = config.day;
    let dday = format!("{:02}", day);

    let cmd = Command::new("cargo")
        .args(["run", "--bin", &format!("day{}", dday), &input_filename])
        .output()
        .unwrap();
    
    let output = String::from_utf8(cmd.stdout).unwrap();
    let vec: Vec<&str> = output.trim().split("\n").collect();
    
    // PRINTS
    startup(day);
    for part in vec {
        println!("{}", part);
    }
    ending();
}
