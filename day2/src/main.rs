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

fn score(a: &str, b: &str) -> i32 {
    if a == "A" && b == "X" || a == "B" && b == "Y" || a == "C" && b == "Z" {
        return 3;
    }
    if a == "A" && b == "Z" || a == "B" && b == "X" || a == "C" && b == "Y" {
        return 0;
    } 
    6
}

fn part1(input: &Vec<String>) -> i32 {
    let mut result = 0;
    for line in input {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match parts[1] {
            "X" => result += 1 + score(parts[0], parts[1]),
            "Y" => result += 2 + score(parts[0], parts[1]),
            "Z" => result += 3 + score(parts[0], parts[1]),
            _ => panic!("Unknown operation"),
        }
        println!("{}: {}", line, result);
    }
    result
}

fn part2(input: &Vec<String>) -> i32 {
    0
}