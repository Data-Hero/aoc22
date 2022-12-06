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

fn alldiff(a: char, b: char, c: char, d: char) -> bool {
    a != b && a != c && a != d && b != c && b != d && c != d
}

fn vec_alldiff(chars: &Vec<char>) -> bool {
    for i in 0..chars.len() {
        for j in i + 1..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    true
}

fn part1(input: &Vec<String>) -> usize {
    let chars: Vec<char> = input[0].chars().collect();
    for (index, character) in chars.into_iter().enumerate() {
        if index < 4 {
            continue;
        }
        let chars_inner: Vec<char> = input[0].chars().collect();
        let a = chars_inner[index - 4];
        let b = chars_inner[index - 3];
        let c = chars_inner[index - 2];
        let d = chars_inner[index - 1];
        if alldiff(a, b, c, d) {
            println!("{}{}{}{}", a, b, c, d);
            println!("marker: {}", character);
            return index;
        }
    }
    usize::MAX
}

fn part2(input: &Vec<String>) -> usize {
    let chars: Vec<char> = input[0].chars().collect();
    for (index, character) in chars.into_iter().enumerate() {
        if index < 14 {
            continue;
        }
        let chars_inner: Vec<char> = input[0].chars().collect();
        let mut marker = Vec::<char>::with_capacity(14);
        for j in 1..15 {
            marker.push(chars_inner[index - j]);
        }
        if vec_alldiff(&marker) {
            println!("marker: {:?}", marker);
            println!("marker: {}", character);
            return index;
        }
    }
    usize::MAX
}
