use std::fs::File;
use std::io::Read;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let mut input = String::new();
    File::open("input/day01.txt").unwrap().read_to_string(&mut input).unwrap();

    let mapped: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut cnt = 0;

    for i in 0..=mapped.len() - 2 {
        if mapped[i] < mapped[i+1] {
            cnt += 1;
        }
    }

    println!("part 1 = {}", cnt)
}

fn part2() {
    let mut input = String::new();
    File::open("input/day01.txt").unwrap().read_to_string(&mut input).unwrap();

    let mapped: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut cnt = 0;

    for i in 0..=mapped.len() - 4 {
        if mapped[i..=i+2].iter().sum::<i32>() < mapped[i+1..=i+3].iter().sum::<i32>() {
            cnt += 1;
        }
    }

    println!("part 2 = {}", cnt)
}
