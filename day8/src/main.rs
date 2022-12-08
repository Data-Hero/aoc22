use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fmt;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

struct Tree {
    x: u16,
    y: u16,
    height: i8,
    mark: bool,
}


impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tree")
         .field("mark", &self.mark)
         .finish()
    }
}




fn part1(input: &Vec<String>) -> u64 {
    let mut visible = 0u64;
    let mut forest = Vec::<Vec<Tree>>::new(); 
    for (index, line) in input.iter().enumerate() {
        let mut row = Vec::<Tree>::new();
        let mut max_height = -1i8;
        
        for (i, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap() as i8;
            let mark = height > max_height;
            if height > max_height {
                visible += 1;
                max_height = height;
            }
            row.push(Tree {
                x: i as u16,
                y: index as u16,
                height: height,
                mark: mark,
            });
        }
        max_height = -1i8;
        for index in (0..row.len()).rev() {
            if row[index].height > max_height {
                if !row[index].mark {
                    visible += 1;
                }
                row[index].mark = true;
                max_height = row[index].height;
            }
        }
        forest.push(row);
    }

    let xSize = forest[0].len();
    let ySize = forest.len();

    for i in (0..xSize) {
        let mut max_height = -1i8;
        for j in (0..ySize) {
            if forest[j][i].height > max_height {
                if !forest[j][i].mark {
                    visible += 1;
                }
                forest[j][i].mark = true;
                max_height = forest[j][i].height;
            }
         }

         max_height = -1i8;
         for j in (0..ySize).rev() {
             if forest[j][i].height > max_height {
                 if !forest[j][i].mark {
                     visible += 1;
                 }
                 forest[j][i].mark = true;
                 max_height = forest[j][i].height;
             }
          }
    }

    for row in forest.iter() {
        for tree in row.iter() {
            if tree.mark {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    visible
}

fn part2(input: &Vec<String>) -> i64 {
    0
}
