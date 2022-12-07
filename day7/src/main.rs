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

#[derive(Debug)]
struct Node {
    name: String,
    size: u64,
}


fn part1(input: &Vec<String>) -> u64 {
    let mut result = 0u64;

    let mut size = 0u64;

    let mut folders = HashMap::<String, Node>::new();
    let mut current_node: &str = "";
    let mut pname = "".to_owned();
    let mut count = 0;
    for line in input {
        count = count + 1;
        if line.contains("$ cd ..") && "/" != current_node {
            let parent_node = folders.get(current_node).unwrap();
            pname = parent_node.name.to_owned();
            current_node = &pname;
            size = 0;
        } else if line.contains("$ cd") {
            let name = line.replace("$ cd ", "");
            if !folders.contains_key(&name) {
                let new_node = Node {
                    size: 0,
                    uid: count.to_string() + "-" + &name,
                    name: name.to_owned(),
                    parent: current_node.to_owned(),
                    children: Vec::new(),
                };
                folders.insert(name, new_node);
            }
            pname = line.replace("$ cd ", "");
            current_node = &pname;
            size = 0;
        } else if line.contains("$ ls") {
            size = 0;
            continue;
        } else if line.contains(&['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'])
            && !line.contains("$")
        {
            let additional_size = line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<u64>()
                .unwrap();

            let cn = folders.get(current_node).unwrap();
            folders.insert(
                current_node.to_string(),
                Node {
                    size: cn.size + additional_size,
                    uid: cn.uid.to_string(),
                    name: cn.name.to_string(),
                    parent: cn.parent.to_string(),
                    children: cn.children.to_vec(),
                },
            );
        } else if line.contains("dir ") {
            let name = line.split_whitespace().nth(1).unwrap();
            let mcn = folders.get(current_node);
            if mcn.is_some() {
                let cn = mcn.unwrap();
                let mut childs = cn.children.to_vec();
                childs.push(name.to_string());
                folders.insert(
                    current_node.to_string(),
                    Node {
                        size: cn.size,
                        uid: cn.uid.to_string(),
                        name: cn.name.to_string(),
                        parent: cn.parent.to_string(),
                        children: childs,
                    },
                );
            } else {
                folders.insert(
                    current_node.to_string(),
                    Node {
                        size: 0,
                        uid: count.to_string() + "-" + name,
                        name: name.to_owned(),
                        parent: current_node.to_owned(),
                        children: Vec::new(),
                    },
                );
            }
        }
    }
    println!("=====================================");
    println!("Root: {:?}", folders.get("/").unwrap());
    let keys = folders.keys();
    for key in keys {
        println!("key: {}", key); 
        
        let val = folders.get(key).unwrap();
        println!("childs: {:?}", val.children); 
        let mut size = val.size;
        
        let mut children_list = Vec::<String>::new();
        for child in val.children.iter() {
            let child_node = folders.get(child).unwrap();
            children_list.push(child_node.name.to_string());
        }
        while children_list.len() > 0 {
            println!("children_list of {}: {:?}", key, children_list);
            let child_node_string = children_list.pop().unwrap();
            let child_node = folders.get(&child_node_string).unwrap();
            size = size + child_node.size;
            for child in child_node.children.iter() {
                let cn = folders.get(child).unwrap();
                if !children_list.contains(&cn.name) {
                    children_list.push(cn.name.to_string());
                }
            }
        }
        if size <= 100000 {
            result = result + size;
        }
    }
    result
}

fn part2(input: &Vec<String>) -> i32 {
    0
}
