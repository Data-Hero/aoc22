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

fn char_to_priority(c: char) -> i32 {
    match c as u16 {
        65..=90 => c as i32 - 38,
        97..=122 => c as i32 - 96,
        _ => panic!("Invalid char"),
    } 
}

fn part1(input: &Vec<String>) -> i32 {
    let mut result = 0i32; 
    for line in input {
        let fh = &line[0..line.len()/2];
        let sh = &line[line.len()/2..line.len()];
        'second: for c1 in fh.chars() {
            for c2 in sh.chars() {
                if c1 == c2 {
                    result += char_to_priority(c1);
                    break 'second;
                }
            }
        }
    }
    result
}

fn part2(input: &Vec<String>) -> i32 {
    let mut result = 0;
    let mut counter = 0;
    while counter < input.len()-2 {
        let a = &input[counter];
        let b = &input[1 + counter];
        let c = &input[2 + counter];
        for ch in a.chars() {
            if b.contains(ch) && c.contains(ch) {
                result += char_to_priority(ch);
                break;
            }
        }
        counter = counter + 3
    }
    result
}

