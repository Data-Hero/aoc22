use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
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
    height: i16,
}


fn part1(input: &Vec<String>) -> i64 {
    let mut graph: Graph<Node, i16, Directed> = Graph::new();
    let mut nodes: HashMap<(u8, u8), NodeIndex> = HashMap::new();
    let mut start: NodeIndex = NodeIndex::new(0);
    let mut goal: NodeIndex = NodeIndex::new(0);
    let input_length = input.len();
    let line_length = input[0].len();

    for (li, line) in input.iter().enumerate() {
        for (bi, byte) in line.as_bytes().iter().enumerate() {
            let height = match byte {
                b'S' => b'a',
                b'E' => b'z',
                x => *x,
            };
            let node_id = graph.add_node(Node {
                height: height as i16,
            });
            nodes.insert((bi as u8, li as u8), node_id);
            if *byte as char == 'S' {
                println!("Start: {:?}, x: {}, y: {}", graph.node_weight(node_id).unwrap(), bi, li);
                start = node_id;
            }
            if *byte as char == 'E' {
                goal = node_id;
            }
        }
    }

    for (li, line) in input.iter().enumerate() {
        for (bi, _) in line.as_bytes().iter().enumerate() {
            let a = *nodes.get(&(bi as u8, li as u8)).unwrap();
            if li > 0 {
                if graph
                    .node_weight(*nodes.get(&(bi as u8, li as u8 - 1)).unwrap())
                    .unwrap()
                    .height
                    - graph.node_weight(a).unwrap().height
                    <= 1
                {
                    graph.add_edge(a, *nodes.get(&(bi as u8, li as u8 - 1)).unwrap(), 1);
                }
            }
            if li + 1 < input_length {
                if graph
                    .node_weight(*nodes.get(&(bi as u8, li as u8 + 1)).unwrap())
                    .unwrap()
                    .height
                    - graph.node_weight(a).unwrap().height
                    <= 1
                {
                    graph.add_edge(a, *nodes.get(&(bi as u8, li as u8 + 1)).unwrap(), 1);
                }
            }
            if bi > 0 {
                if graph
                    .node_weight(*nodes.get(&(bi as u8 - 1, li as u8)).unwrap())
                    .unwrap()
                    .height
                    - graph.node_weight(a).unwrap().height
                    <= 1
                {
                    graph.add_edge(a, *nodes.get(&(bi as u8 - 1, li as u8)).unwrap(), 1);
                }
            }
            if bi + 1 < line_length {
                if graph
                    .node_weight(*nodes.get(&(bi as u8 + 1, li as u8)).unwrap())
                    .unwrap()
                    .height
                    - graph.node_weight(a).unwrap().height
                    <= 1
                {
                    graph.add_edge(a, *nodes.get(&(bi as u8 + 1, li as u8)).unwrap(), 1);
                }
            }
        }
    }

    let res = dijkstra(&graph, start, Some(goal), |_| 1);
    *res.iter().filter(|(k, _)| **k == goal).next().unwrap().1
}

fn part2(input: &Vec<String>) -> i64 {
    0
}
