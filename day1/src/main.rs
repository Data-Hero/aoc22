use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("Input:  {:?}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &Vec<String>) -> i64 {
    let mut max = 0i64;
    let mut current = 0i64;
    for line in input {
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            let a = line.parse::<i64>().unwrap();
            current += a;
        }
    }
    max
}

fn part2(input: &Vec<String>) -> i64 {
    let mut heap = BinaryHeap::<i64>::new();
    let mut current = 0i64;
    for line in input {
        if line.is_empty() {
            heap.push(current);
            current = 0;
        } else {
            let a = line.parse::<i64>().unwrap();
            current += a;
        }
    }
    let a = heap.pop().unwrap();
    let b = heap.pop().unwrap();
    let c = heap.pop().unwrap();
    a+b+c
}
