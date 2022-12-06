use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const HEIGHT_SUPREMUM: usize = 8;

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

fn parse_start(input: &Vec<String>, stacks: &mut Vec<Vec<char>>) -> io::Result<()> {
   for j in (0..HEIGHT_SUPREMUM).rev() {
        let mut lineWithTwoNone = input[j]
            .split(" ")
            .map(|e| e
                .replace("[", "")
                .replace("]", "")
                .chars().next())
            .collect::<Vec<Option<char>>>();
        let mut line = Vec::<Option<char>>::new();
        while !lineWithTwoNone.is_empty() {
            let element = lineWithTwoNone.pop().unwrap();
            match element {
                Some(c) => line.push(Some(c)),
                None => { 
                    line.push(None);
                    lineWithTwoNone.pop();
                    lineWithTwoNone.pop();
                    lineWithTwoNone.pop();
                },
            }
        }
        for i in 0..line.len() {
            if line[i].is_some() {
                stacks[line.len()-i-1].push(line[i].unwrap());
            }
        }
   }
   Ok(())
}

fn parse_line(line: &str) -> (usize, usize, usize) {
    let a = line
            .trim()
            .split(" ")
            .filter(|x| {
                *x != "move" && *x != "to" && *x != "from" 
            })
            .map(|x| {
                if x != "" { return x.parse::<usize>().unwrap(); } else { return 0; } 
            })
            .collect::<Vec<usize>>();
    (a[0], a[1], a[2])
}

fn part1(input: &Vec<String>) -> String {
    let number_of_stacks = max(&input[HEIGHT_SUPREMUM]
        .replace(" ", "")
        .split("")
        .map(|x| {
            if x != "" { return x.parse::<i32>().unwrap(); } else { return 0; } 
        })
        .collect::<Vec<i32>>());
    let mut stacks = vec![Vec::<char>::with_capacity(HEIGHT_SUPREMUM as usize); number_of_stacks as usize];
    parse_start(input, &mut stacks);
    println!("Stacks start:\n {:?}", stacks);
    for (count, line) in input.iter().enumerate() {
        if count <= HEIGHT_SUPREMUM + 1{
            continue;
        }
        let (amount, from, to) = parse_line(line);
        for i in 0..amount {
            let element = stacks[from-1].pop().unwrap();
            stacks[to-1].push(element);
            
        }
    }
    println!("Stacks finished:\n {:?}", stacks);
    let mut result = "".to_string();
    for i in 0..stacks.len() {
        result.push(stacks[i].pop().unwrap());
    }
    result
}

fn part2(input: &Vec<String>) -> i32 {
    0
}
