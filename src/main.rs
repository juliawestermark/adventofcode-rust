use std::{
    env,
    fs::File,
    process::Command,

};

use chrono::{
    NaiveDate,
    Utc,
    Months,
    Datelike,
};

use aoc2022::{
    ANSI_ITALIC,
    ANSI_BOLD,
    ANSI_RESET,
    COLOR_GREEN,
    print_fail,
};

fn startup(day: i32) {
    println!();
    println!("{}--- 🎄 Day {:02} 🎄 ---{}", ANSI_BOLD, day, ANSI_RESET);
    println!();
}

fn ending() {
    let from_ymd_opt = NaiveDate::from_ymd_opt;
    let now = Utc::now();
    let today = from_ymd_opt(now.year(), now.month(), now.day()).unwrap();
    let christmas_date = from_ymd_opt(now.year(), 12, 24).unwrap(); 
    // let today = from_ymd_opt(2022, 12, 02).unwrap();

    let diff = date_diff(today, christmas_date);

    println!();
    println!("{}--- 🎄🎄🎄🎄🎄🎄 ---{}", ANSI_BOLD, ANSI_RESET);
    if diff == 0 {
        println!("{}   IT'S CHRISTMAS   {}", COLOR_GREEN, ANSI_RESET);
    }
    else if diff == 1 {
        println!("{}{} day to Christmas!!{}", COLOR_GREEN, diff, ANSI_RESET);
    }
    else if diff < 10 {
        println!("{}{} days to Christmas!{}", COLOR_GREEN, diff, ANSI_RESET);
    }
    else {
        println!("{}{} days to Christmas{}", COLOR_GREEN, diff, ANSI_RESET);
    }
    println!("{}--- 🎄🎄🎄🎄🎄🎄 ---{}", ANSI_BOLD, ANSI_RESET);
    println!();
}

fn date_diff(start_date: NaiveDate, end_date: NaiveDate) -> i64{
    let d = (end_date - start_date).num_days();
    if d < 0 {
        let new_date = end_date + Months::new(12);
        return date_diff(start_date, new_date)
    }
    return d
}

struct Config {
    day: i32,
    input_filename: String,
}

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
            "input" => {
                input_filename = format!("src/inputs/input{:02}.txt", day).to_string();
            },
            "example" => {
                input_filename = format!("src/examples/example{:02}.txt", day).to_string();
            },
            _ => {
                input_filename = format!("src/inputs/input{:02}.txt", day).to_string();
            },
        }
    }

    Config {day, input_filename}

}

fn file_exists(filename: String) -> bool {
    match File::open(&filename) {
        Ok(_) => {
            true
        },
        Err(_) => {
            let message = format!("File does not exist \"{}\"", &filename);
            print_fail(message);
            false
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let input_filename = config.input_filename;
    let day = config.day;
    let dday = format!("day{:02}", &day);
    let path_bin = "src/bin";

    startup(day);

    if file_exists(format!("{}/{}.rs",path_bin, dday)) {
        if file_exists(input_filename.clone()) {
            let cmd = Command::new("cargo")
                .args(["run", "--bin", &dday, &input_filename])
                .output()
                .unwrap();
            
            let output = String::from_utf8(cmd.stdout).unwrap();
            let vec: Vec<&str> = output.trim().split("\n").collect();

            println!("{}Run for input {}{}", ANSI_ITALIC, input_filename, ANSI_RESET);
            println!();

            if vec.len() == 0 || vec[0].is_empty() {
                println!("No solutions");
            }
            else {
                for part in vec {
                    println!("{}", part);
                }
            }
        }
    }

    ending();
}
