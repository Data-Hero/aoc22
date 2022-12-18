use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fmt;

const X_SHIFT: i32 = -450i32;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Sand = 0,
    Air = 1,
    Rock = 2,
    Entry = 3,
}

struct Cave {
    map: [[Element; 200]; 100],
    origin: Vec<(usize, usize)>,
    dropped: usize,
}

impl Cave {
    fn new() -> Cave {
        let mut cave = Cave {
            map: [[Element::Air; 200]; 100],
            origin: vec![(50, 0)],
            dropped: 0,
        };
        cave.map[50][0] = Element::Entry;
        cave
    }

    fn add_path(&mut self, path: Vec<(usize, usize)>) {
        if path.is_empty() {
            return;
        }

        self.map[path[0].0][path[0].1] = Element::Rock;
        for i in 0..path.len() - 1 {
            if path[i].1 != path [i + 1].1 {
                // vertical
                if path[i].1 < path[i + 1].1 {
                    // down
                    for j in path[i].1..=path[i + 1].1 {
                        self.map[path[i].0][j] = Element::Rock;
                    }
                } else {
                    // up
                    for j in path[i + 1].1..=path[i].1 {
                        self.map[path[i].0][j] = Element::Rock;
                    }
                }
            } else {
                // horizontal
                if path[i].0 < path[i + 1].0 {
                    // right
                    for j in path[i].0..=path[i + 1].0 {
                        self.map[j][path[i].1] = Element::Rock;
                    }
                } else {
                    // left
                    for j in path[i + 1].0..=path[i].0 {
                        self.map[j][path[i].1] = Element::Rock;
                    }
                }
            }
        }
    }

    fn drop(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y >= 198 {
            return None;
        }
    
        if self.map[x][y + 1] == Element::Air {
            return self.drop(x, y + 1);
        } else {
            if self.map[x - 1][y + 1] == Element::Air {
                return self.drop(x - 1, y);
            } else if self.map[x + 1][y + 1] == Element::Air {
                return self.drop(x + 1, y);
            } 
        }
        Some((x,y))
    }
    

}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut lines = String::new();
        for y in 0..200 {
            for x in 0..100 {
                match self.map[x][y] {
                    Element::Sand => s.push('o'),
                    Element::Air => s.push('.'),
                    Element::Rock => s.push('#'),
                    Element::Entry => s.push('+'),
                }
            }
            lines.push_str(&format!("{:?}\n", s));
            s = String::new();
        }
        write!(f, "[\n{}]", lines)
    }
}


fn part1(input: &Vec<String>) -> usize {
    // Cave creation
    // 0 is 450, 100 ist 550 in width, height is counted from 0 (top) to 100 (bottom)
    let mut cave = Cave::new();
    for line in input {
        let str_path = line.replace("->", ",");
        let split_path = str_path
            .split(",")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut vpath = Vec::<(usize, usize)>::new(); 
        for j in (0..split_path.len()).step_by(2) {
            let x = split_path[j] as i32 + X_SHIFT;
            vpath.push((x as usize, split_path[j + 1]));
        }
        cave.add_path(vpath);
    }

    // Sand drop simulation
    let mut result = 0usize;
    while cave.dropped == 0 {
        for origin in cave.origin.iter() {
            let position = cave.drop(origin.0, origin.1);
            match position {
                Some(p) => {
                    cave.map[p.0][p.1] = Element::Sand;
                    result += 1;
                },
                None => {
                    cave.dropped += 1;
                }
            }

        }
    }
    println!("Cave end: {:?}", cave);
    result
}

fn part2(input: &Vec<String>) -> usize {
    0
}
