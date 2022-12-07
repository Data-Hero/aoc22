use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::iter::FromIterator;
use std::cmp::{min,max};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Node {
    name: String,
    size: u64,
}

fn part1(input: &Vec<String>) -> u64 {
    let mut result = 0u64;

    let mut folders = BTreeMap::<String, Node>::new();
    let mut current_path = Vec::<String>::new();
    for line in input {
        if line.contains("$ cd ..") && current_path.len() > 1 {
            current_path.pop();
        } else if line.contains("$ cd") {
            let name = line.replace("$ cd ", "");
            current_path.push(name.to_string());
            if !folders.contains_key(&name) {
                let new_node = Node {
                    name: current_path.join("/"),
                    size: 0,
                };
                folders.insert(current_path.join("/"), new_node);
            }
        } else if line.contains(&['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'])
            && !line.contains("$")
        {
            let additional_size = line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let cn = folders.get(&current_path.join("/")).unwrap();
            folders.insert(
                current_path.join("/"),
                Node {
                    name: cn.name.to_string(),
                    size: cn.size + additional_size,
                },
            );
        }
    }

    let keys = folders.keys();
    for key in keys {
        let val = folders.get(key).unwrap();
        let mut s = val.size;
        for (key2, val2) in &folders {
            if key != key2 && key2.contains(key) {
                s = s + folders.get(key2).unwrap().size;
            }
        }
        if s <= 100000 {
            result = result + s;
        }
    }
    result
}

fn part2(input: &Vec<String>) -> u64 {
    let mut folders = BTreeMap::<String, Node>::new();
    let mut current_path = Vec::<String>::new();
    for line in input {
        if line.contains("$ cd ..") && current_path.len() > 1 {
            current_path.pop();
        } else if line.contains("$ cd") {
            let name = line.replace("$ cd ", "");
            current_path.push(name.to_string());
            if !folders.contains_key(&name) {
                let new_node = Node {
                    name: current_path.join("/"),
                    size: 0,
                };
                folders.insert(current_path.join("/"), new_node);
            }
        } else if line.contains(&['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'])
            && !line.contains("$")
        {
            let additional_size = line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let cn = folders.get(&current_path.join("/")).unwrap();
            folders.insert(
                current_path.join("/"),
                Node {
                    name: cn.name.to_string(),
                    size: cn.size + additional_size,
                },
            );
        }
    }

    let size_vec = folders.values().map(|n| n.size).collect::<Vec<u64>>();
    let required = max(40000000, size_vec.iter().sum::<u64>()) - min(40000000, size_vec.iter().sum::<u64>());
    let mut min = Node {
        name: "".to_string(),
        size: u64::MAX,
    };
    
    let keys = folders.keys();
    for key in keys {
        let val = folders.get(key).unwrap();
        let mut s = val.size;
        for (key2, val2) in &folders {
            if key != key2 && key2.contains(key) {
                s = s + folders.get(key2).unwrap().size;
            }
        }
        if s <= min.size && s >= required {
            min = Node {
                name: key.to_string(),
                size: s,
            };
        }
    }
    min.size
}
