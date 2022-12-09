use std::collections::HashMap;
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

fn signum(a: i64) -> i64 {
    if a > 0 {
        1i64
    } else if a < 0 {
        -1i64
    } else {
        0i64
    }
}

#[derive(Copy, Clone)]
struct KnotPosition {
    x: i64,
    y: i64,
}

impl KnotPosition {
    fn step(&mut self, direction: &str) {
        match direction {
            "D" => self.y -= 1i64,
            "U" => self.y += 1i64,
            "L" => self.x -= 1i64,
            "R" => self.x += 1i64,
            _ => panic!("Unknown direction: {}", direction),
        }
    }
    fn to(&mut self, second: &KnotPosition) -> bool {
        if self.inf_dist(second) != 2i64 {
            return false;
        }
        if self.x + 2 == second.x {
            self.x += 1i64;
            self.y += signum(second.y - self.y);
            return true;
        }
        if self.x - 2 == second.x {
            self.x -= 1i64;
            self.y += signum(second.y - self.y);
            return true;
        }
        if self.y + 2 == second.y {
            self.y += 1i64;
            self.x += signum(second.x - self.x);
            return true;
        }
        if self.y - 2 == second.y {
            self.y -= 1i64;
            self.x += signum(second.x - self.x);
            return true;
        }
        false
    }
    fn inf_dist(&self, second: &KnotPosition) -> i64 {
        (self.x - second.x).abs().max((self.y - second.y).abs())
    }
    fn m_dist(&self, second: &KnotPosition) -> i64 {
        (self.x - second.x).abs() + (self.y - second.y).abs()
    }
}

fn part1(input: &Vec<String>) -> usize {
    let mut tail = KnotPosition { x: 300, y: 300 };
    let mut head = KnotPosition { x: 300, y: 300 };
    let mut map = HashMap::<i64, bool>::new();
    for line in input {
        let step = line.split_whitespace().collect::<Vec<&str>>();
        let direction = step[0];
        let distance = step[1].parse::<u64>().unwrap();
        let mut c = 0u64;
        while c < distance {
            head.step(direction);
            let moved = tail.to(&head);
            if moved {
                map.insert(1000 * tail.x + tail.y, true);
            } 
            c += 1;
        }
    }
    map.keys().len()
}

fn part2(input: &Vec<String>) -> usize {
    let mut knots = [KnotPosition { x: 300, y: 300 }; 10];
    let mut map = HashMap::<i64, bool>::new();
    for line in input {
        let step = line.split_whitespace().collect::<Vec<&str>>();
        let direction = step[0];
        let distance = step[1].parse::<u64>().unwrap();
        let mut c = 0u64;
        while c < distance {
            knots[0].step(direction);
            for j in 1..10 {
                let mut next = knots[j];
                let last = knots[j - 1];
                let moved = next.to(&last);
                if moved {
                    knots[j] = next;
                    if j == 9 {
                        map.insert(1000 * next.x + next.y, true);
                    }
                } 
            }
            c += 1;
        }
    }
    map.keys().len()
}
