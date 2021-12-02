use std::fs::File;
use std::io::Read;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let mut input = String::new();
    File::open("input/day02.txt").unwrap().read_to_string(&mut input).unwrap();

    let mut x = 0;
    let mut depth = 0;

    for line in input.lines() {
        let splits: Vec<_> = line.split(' ').collect();
        let amount = splits[1].parse::<i32>().expect("Invalid line");
        match splits[0] {
            "up" => depth -= amount,
            "down" => depth += amount,
            "forward" => x += amount,
            &_ => {}
        }
    }

    let res = x * depth;
    println!("part 1 = {}", res)
}

fn part2() {
    let mut input = String::new();
    File::open("input/day02.txt").unwrap().read_to_string(&mut input).unwrap();

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let splits: Vec<_> = line.split(' ').collect();
        let amount = splits[1].parse::<i32>().expect("Invalid line");
        match splits[0] {
            "up" => aim -= amount,
            "down" => aim += amount,
            "forward" => {
                x += amount;
                depth += amount * aim;
            },
            &_ => {}
        }
    }

    let res = x * depth;
    println!("part 1 = {}", res)
}
