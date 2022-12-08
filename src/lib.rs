use std::{
    env,
    fs::{self, File},
    io::{prelude::*, BufReader},
    path::Path,
    process,
};

use chrono::{
    NaiveDate,
    Utc,
    Months,
    Datelike,
};


pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";
pub const COLOR_RED: &str = "\x1b[0;31m";
pub const COLOR_GREEN: &str = "\x1b[0;32m";
pub const COLOR_GOLD: &str = "\x1b[0;33m";
pub const COLOR_LIGHT_GRAY: &str = "\x1b[0;90m";


pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn string_from_file(filename: impl AsRef<Path>) -> String {
    fs::read_to_string(filename)
        .expect("Should have been able to read the file")
}

pub fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough arguments");
    }
    
    let filename: String = match args[1].parse() {
        Ok(filename) => filename,
        Err(_) => {
            eprintln!("Input file not specified`");
            process::exit(1);
        }
    };
    filename
}

fn date_diff(start_date: NaiveDate, end_date: NaiveDate) -> i64{
    let d = (end_date - start_date).num_days();
    if d < 0 {
        let new_date = end_date + Months::new(12);
        return date_diff(start_date, new_date)
    }
    return d
}

pub fn print_success(message: String) {
    println!("{}{}{}", COLOR_GREEN, message, ANSI_RESET);
}

pub fn print_fail(message: String) {
    println!("{}{}{}", COLOR_RED, message, ANSI_RESET);
}

pub fn print_startup(day: i32) {
    println!();
    println!("{}--- 🎄 Day {:02} 🎄 ---{}", ANSI_BOLD, day, ANSI_RESET);
    println!();
}

pub fn christmas_carol(song_nbr: i32) -> String {
    let filename = "src/songs.txt";
    let input = string_from_file(filename);
    let mut song_nbr_usize: usize = song_nbr.try_into().unwrap();
    let vec: Vec<&str> = input.split("#").collect();
    if vec.len() < 1 {
       return "What do you get when you cross a snowman with a vampire? Frostbite.".to_string()
    }
    else if vec.len() < song_nbr_usize {
        song_nbr_usize = 1;
    }
    vec.get(song_nbr_usize-1).expect("Could not convert song").to_string()
}

pub fn today() -> NaiveDate {
    let from_ymd_opt = NaiveDate::from_ymd_opt;
    let now = Utc::now();
    let today = from_ymd_opt(now.year(), now.month(), now.day()).unwrap();
    return today
}

pub fn print_ending() {
    let from_ymd_opt = NaiveDate::from_ymd_opt;
    let today = today();
    let christmas_date = from_ymd_opt(today.year(), 12, 24).unwrap(); 
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

pub fn print_advent_of_code() {
    println!();
    println!("{}🎄 Advent of code 🎄{}", ANSI_BOLD, ANSI_RESET);
    println!();
}

pub fn get_days_in_december() -> i32 {
    let from_ymd_opt = NaiveDate::from_ymd_opt;
    let today = today();
    let december_first = from_ymd_opt(2022, 11, 30).unwrap(); // for 2022
    
    let date_diff = (today - december_first).num_days() as i32;
    date_diff
}
