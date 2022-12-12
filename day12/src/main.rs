use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use petgraph::Graph;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::HashMap;


fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    let mut graph : Graph<(),(),Directed>= Graph::new();
    let a = graph.add_node(()); // node with no weight
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    let e = graph.add_node(());
    let f = graph.add_node(());
    let g = graph.add_node(());
    let h = graph.add_node(());
    // z will be in another connected component
    let z = graph.add_node(());
    
    graph.extend_with_edges(&[
        (a, b),
        (b, c),
        (c, d),
        (d, a),
        (e, f),
        (b, e),
        (f, g),
        (g, h),
        (h, e)
    ]);
    // a ----> b ----> e ----> f
    // ^       |       ^       |
    // |       v       |       v
    // d <---- c       h <---- g
    
    let expected_res: HashMap<NodeIndex, usize> = [
         (a, 3),
         (b, 0),
         (c, 1),
         (d, 2),
         (e, 1),
         (f, 2),
         (g, 3),
         (h, 4)
        ].iter().cloned().collect();
    let res = dijkstra(&graph,b,None, |_| 1);
    assert_eq!(res, expected_res);
    println!("Graph: {:?}", graph);
    println!("Dijkstra: {:?}", res);

    Ok(())
}




fn part1(input: &Vec<String>) -> i64 {
    let mut graph: Graph<u8, u8, Directed> = Graph::new();
    let linelength = input[0].len();

    for line in input {
        for b in line.as_bytes() {
            graph.add_node(*b);
        }
    }
    println!("Graph: {:?}", graph);
    for (li, line) in input.iter().enumerate() {
        for (bi, byte) in line.as_bytes().iter().enumerate() {
            if li - 1 > 0 {

            }
        }
    }
    0
}

fn part2(input: &Vec<String>) -> i64 {
    0
}
