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

fn contain(s1: &str, s2: &str) -> bool {
    let n1 = s1.split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let n2 = s2.split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if n1[0] <= n2[0] && n1[1] >= n2[1] {
        return true;
    }
    false
}

fn part1(input: &Vec<String>) -> i32 {
    let mut result = 0;
    for l in input {
        let mut parts = l.split(",").collect::<Vec<&str>>();
        if contain(parts[0],parts[1]) || contain(parts[1], parts[0]) {
            println!("{} contains {}", parts[0], parts[1]);
            result += 1;
        }
    }
    result
}

fn part2(input: &Vec<String>) -> i32 {
    0
}