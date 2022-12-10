use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut actions = Vec::<Option<i64>>::new();

    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    for line in input {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        if line.len() == 2 {
            actions.push(None);
            actions.push(Some(line[1].parse::<i64>().unwrap()));
        } else {
            actions.push(None);
        }
    }
    println!("Input: {:?}", actions);
    println!("Part 1: {}", part1(&actions));
    println!("Part 2: {}", part2(&actions));
    Ok(())
}

fn part1(input: &Vec<Option<i64>>) -> i64 {
    let mut result = 0i64;
    let mut x = 1i64;
    for (index, value) in input.iter().enumerate() {
        if (index + 21) % 40 == 0 {
            result += x * (index + 1) as i64;
        }
        x += match value {
            Some(value) => value,
            None => &0i64,
        };
    }
    result
}

fn part2(input: &Vec<Option<i64>>) -> String {
    let mut drawing = [['.'; 40]; 6];
    let mut x = 1i64;
    for (index, value) in input.iter().enumerate() {
        drawing[index / 40][index % 40] = {
            if (index % 40) + 1 >= x as usize && (index % 40) <= x as usize + 1 || index % 40 == 0 {
                '#'
            } else {
                '.'
            }
        };

        x += match value {
            Some(value) => value,
            None => &0i64,
        };
    }

    let mut result = "\n".to_string();
    for pixel in drawing {
        for value in pixel {
            result.push(value);
        }
        result.push('\n');
    }
    result
}
