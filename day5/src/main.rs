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

fn max(vector: &Vec<i32>) -> i32 {
    let mut max = 0;
    for v in vector {
        if *v > max {
            max = *v;
        }
    }
    max
}


fn part1(input: &Vec<String>) -> i32 {
    let number_of_stacks = max(&input[8]
        .replace(" ", "")
        .split("")
        .map(|x| {
            if x != "" { return x.parse::<i32>().unwrap(); } else { return 0; } 
        })
        .collect::<Vec<i32>>());
    let mut stacks = vec![vec![]; number_of_stacks as usize];
    number_of_stacks
}

fn part2(input: &Vec<String>) -> i64 {
    0
}
