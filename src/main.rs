mod day01;

use std::env;
// use std::fs::File;
use std::io::Error;
// use std::io::{BufRead, BufReader, Error, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let default_day = 1;

    let day: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(default_day)
    } else {
        default_day
    };

    run_day(day)?;

    return Ok(());
}

fn run_day(day: i32) -> Result<(), Error> {
    // let file = input_file_for_day(day);
    // let buf_reader: dyn BufRead = match file {
    //     Ok(file) => BufReader::new(file),
    //     _ => std::io::stdin().lock(),
    // };

    match day {
        1 => day01::run(),
        _ => println!("Day {} not yet implemented!", day),
    };

    Ok(())
}

// fn input_file_for_day(day: i32) -> std::io::Result<File> {
//     File::open(format!("input/day{:02}.txt", day))
// }
