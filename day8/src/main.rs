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

struct Tree {
    x: u16,
    y: u16,
    height: i8,
    mark: bool,
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
    visible
}

fn part2(input: &Vec<String>) -> u64 {
    let mut rating = 0u64;

    let mut forest = Vec::<Vec<Tree>>::new();
    for (index, line) in input.iter().enumerate() {
        let mut row = Vec::<Tree>::new();
        for (i, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap() as i8;
            row.push(Tree {
                x: i as u16,
                y: index as u16,
                height: height,
                mark: false,
            });
        }
        forest.push(row);
    }

    let xSize = forest[0].len();
    let ySize = forest.len();

    for i in (0..xSize) {
        let mut max_height = -1i8;
        for j in (0..ySize) {
            let mut left_rating = 0u64;
            let mut top_rating = 0u64;
            let mut right_rating = 0u64;
            let mut bottom_rating = 0u64;
            // wander left
            for x in (0..i).rev() {
                left_rating += 1;
                if forest[j][x].height >= forest[j][i].height {
                    break;
                }
            }
            // check right
            for x in (i..xSize) {
                if x == i {
                    continue;
                }
                right_rating += 1;
                if forest[j][x].height >= forest[j][i].height {
                    break;
                }
            }
            // check top
            for y in (0..j).rev() {
                top_rating += 1;
                if forest[y][i].height >= forest[j][i].height {
                    break;
                }
            }
            // check bottom
            for y in (j..ySize) {
                if y == j {
                    continue;
                }
                bottom_rating += 1;
                if forest[y][i].height >= forest[j][i].height {
                    break;
                }
            }
            let tree_rating = left_rating * top_rating * right_rating * bottom_rating;
            if tree_rating > rating {
                rating = tree_rating;
            }
        }
    }
    rating
}
