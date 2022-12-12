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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_traversable(direction: Direction, x: usize, y: usize, input: &Vec<String>, nodes: &HashMap<(u8,u8), NodeIndex>, graph: &Graph<Node, i16, Directed>, cn: &NodeIndex) -> bool {
    let max_x = input[0].len();
    let max_y = input.len();
    return match direction {
        Direction::Left => x > 0 && graph
                    .node_weight(*nodes.get(&(x as u8 - 1, y as u8)).unwrap())
                    .unwrap()
                    .height
                    - graph.node_weight(*cn).unwrap().height
                    <= 1,
        Direction::Right => x + 1 < max_x && graph
                    .node_weight(*nodes.get(&(x as u8 + 1, y as u8)).unwrap())
                    .unwrap()
                    .height
                    - graph.node_weight(*cn).unwrap().height
                    <= 1,
        Direction::Up => y > 0 && graph
                    .node_weight(*nodes.get(&(x as u8, y as u8 - 1)).unwrap())
                    .unwrap()
                    .height
                    - graph.node_weight(*cn).unwrap().height
                    <= 1,
        Direction::Down => y + 1 < max_y && graph
                    .node_weight(*nodes.get(&(x as u8, y as u8 + 1)).unwrap())
                    .unwrap()
                    .height
                    - graph.node_weight(*cn).unwrap().height
                    <= 1,
    };
}

fn part1(input: &Vec<String>) -> i64 {
    let mut graph: Graph<Node, i16, Directed> = Graph::new();
    let mut nodes: HashMap<(u8, u8), NodeIndex> = HashMap::new();
    let mut start: NodeIndex = NodeIndex::new(0);
    let mut goal: NodeIndex = NodeIndex::new(0);

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
                start = node_id;
            }
            if *byte as char == 'E' {
                goal = node_id;
            }
        }
    }

    for (li, line) in input.iter().enumerate() {
        for (bi, _) in line.as_bytes().iter().enumerate() {
            let cn = *nodes.get(&(bi as u8, li as u8)).unwrap();
            if is_traversable(Direction::Up, bi, li, input, &nodes, &graph, &cn) {
                graph.add_edge(cn, *nodes.get(&(bi as u8, li as u8 - 1)).unwrap(), 1);
            }
            if is_traversable(Direction::Down, bi, li, input, &nodes, &graph, &cn) {
                graph.add_edge(cn, *nodes.get(&(bi as u8, li as u8 + 1)).unwrap(), 1);
            }
            if is_traversable(Direction::Left, bi, li, input, &nodes, &graph, &cn) {
                graph.add_edge(cn, *nodes.get(&(bi as u8 - 1, li as u8)).unwrap(), 1);
            }
            if is_traversable(Direction::Right, bi, li, input, &nodes, &graph, &cn) {
                graph.add_edge(cn, *nodes.get(&(bi as u8 + 1, li as u8)).unwrap(), 1);
            }
        }
    }

    let res = dijkstra(&graph, start, Some(goal), |_| 1);
    *res.iter().filter(|(k, _)| **k == goal).next().unwrap().1
}

fn part2(input: &Vec<String>) -> i64 {
    0
}
