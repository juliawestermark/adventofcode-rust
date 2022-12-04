# Advent of code in Rust 2022

> **_NOTE:_** The `<day>` parameter in the files should start with a leading zero if it's only a single letter.

## Edit code

Your solution code should be saved in folder `src/bin/` with name `day<day>.rs`, e.g. `day01.rs` for the first day of december. Example input should be added to folder `src/examples/` with name `example<day>.txt` and input should be added to folder `src/inputs/` with name `input<day>.txt`.


## Create files
Run
```
cargo create <day>
```
to create run file from template and empty input and example files for the specified day.


## Run script
### Production
Run
```
cargo run <day> [input|example]
```
to run the file for the specified day. The last argument is optional and specifies which input file to run. `input` is default.

To run **all** days type
```
cargo run all [input|example]
```
`input` is default.

### Testing
Sometimes you must run the file directly. The compilation errors does not seem to show if running the other script. Do this by running:
```
cargo run --bin <file> <filepath>
```
where `<file>` is the name of the file without extention. The argument `filepath` is either `src/inputs/input<day>.txt` or `src/examples/example<day>.txt`.

So for day one example file write
```
cargo run --bin day01 src/examples/example01.txt
```


## Results
### Print results
Run
```
cargo result
```
to print the result table.

### Edit results
Atm you must manually edit the results file. The first number in each row is the day. The second number is the number of stars for that day. If you have solved the first part of the day, change this to 1. If you have solved both parts for that day, type 2.


## Installation
Install Rust [here](https://www.rust-lang.org/).
