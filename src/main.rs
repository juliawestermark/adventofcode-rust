use std::{
    env,
    fs::File,
    process::Command,
};

use aoc2022::{
    ANSI_ITALIC,
    ANSI_RESET,
    print_fail,
    print_startup,
    print_ending,
    get_days_in_december,
};


struct Config {
    day: i32,
    input_type: String,
}

fn parse_config(args: &[String]) -> Config {
    let input_type;

    if args.len() < 2 {
        panic!("not enough arguments");
    }
    else if args.len() < 3 {
        input_type = "input".to_string();
    }
    else {
        input_type = args[2].to_string();
    }

    let d = args[1].clone();

    let day: i32 = match d.as_str() {
        "all" => 0,
        _ => {
            match d.parse::<i32>() {
                Ok(dday) => {
                    if dday < 1 || dday > 25 {
                        panic!("argument day is not in interval");
                    }
                    dday
                },
                Err(_)=> panic!("argument day is invalid"),
            }
        },
    };

    Config {day, input_type}

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

fn get_filename(day: i32, input_type: String) -> String {
    let input_name = match input_type.as_str() {
        "input" => {
            format!("/inputs/input")
        },
        "example" => {
            format!("/examples/example")
        },
        _ => {
            format!("/inputs/input")
        },
    };
    format!("src{}{:02}.txt", input_name, day).to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let input_type = config.input_type;
    let day_arg = config.day;
    let path_bin = "src/bin";

    
    
    let mut days: Vec<i32>;
    if day_arg == 0 {
        let end_date = get_days_in_december();

        days = (1..end_date+1).collect();
    }
    else {
        days = Vec::new();
        days.push(day_arg);
    }
    
    for day in days {
        let dday = format!("day{:02}", &day);
        let input_filename = get_filename(day, input_type.clone());
        
        print_startup(day);
        
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
    }

    print_ending();
}
