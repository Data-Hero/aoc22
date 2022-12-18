use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &Vec<String>) -> i64 {
    for line in input {
        let r = line.replace("->", ",");
        let s = r.split(",").map(|s| s.trim().parse::<i32>().unwrap() ).collect::<Vec<i32>>();
        println!("{:?}", s)
    }
    0
}

fn part2(input: &Vec<String>) -> i64 {
    0
}
