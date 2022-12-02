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
    }
    result
}

fn choose_win(a: &str) -> i32 {
    match a {
        "A" => 2,
        "B" => 3,
        "C" => 1,
        _ => panic!("Unknown operation"),
    }
}
fn choose_draw(a: &str) -> i32 {
    match a {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Unknown operation"),
    }
}

fn choose_loss(a: &str) -> i32 {
    match a {
        "A" => 3,
        "B" => 1,
        "C" => 2,
        _ => panic!("Unknown operation"),
    }
}

fn part2(input: &Vec<String>) -> i32 {
    let mut result = 0;
    for line in input {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match parts[1] {
            "X" => result += 0 + choose_loss(parts[0]),
            "Y" => result += 3 + choose_draw(parts[0]),
            "Z" => result += 6 + choose_win(parts[0]),
            _ => panic!("Unknown operation"),
        }
    }
    result
}
