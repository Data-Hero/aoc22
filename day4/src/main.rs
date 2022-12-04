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

struct Assignment {
    x: i32,
    y: i32,
}

impl Assignment {
    fn new(x: &str, y: &str) -> Assignment {
        Assignment {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
        }
    }
}

trait Contains {
    fn contains(&self, other: &Assignment) -> bool;
}

trait Overlaps {
    fn overlaps(&self, other: &Assignment) -> bool;
}

impl Contains for Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.x <= other.x && self.y >= other.y
    }
}

impl Overlaps for Assignment {
    fn overlaps(&self, other: &Assignment) -> bool {
        self.x <= other.x && self.y >= other.x || other.x <= self.x && other.y >= self.x
    }
}

fn part1(input: &Vec<String>) -> i32 {
    let mut result = 0;
    for l in input {
        let parts = l
            .split(",")
            .map(|e| {
                let is = e.split("-").collect::<Vec<&str>>();
                Assignment::new(is[0], is[1])
            })
            .collect::<Vec<Assignment>>();

        if parts[0].contains(&parts[1]) || parts[1].contains(&parts[0]) {
            result += 1;
        }
    }
    result
}

fn part2(input: &Vec<String>) -> i32 {
    let mut result = 0;
    for l in input {
        let parts = l
            .split(",")
            .map(|e| {
                let is = e.split("-").collect::<Vec<&str>>();
                Assignment::new(is[0], is[1])
            })
            .collect::<Vec<Assignment>>();

        if parts[0].overlaps(&parts[1]) {
            result += 1;
        }
    }
    result
}
