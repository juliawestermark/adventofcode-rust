use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process,
};


pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";
pub const COLOR_RED: &str = "\x1b[0;31m";
pub const COLOR_GREEN: &str = "\x1b[0;32m";


pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
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
