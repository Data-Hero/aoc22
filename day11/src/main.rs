#![allow(unused_assignments)]
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const NUMBER_OF_ROUNDS_1: usize = 20;
const NUMBER_OF_ROUNDS_2: usize = 10000;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|l| l.unwrap().trim().to_string())
        .collect::<Vec<String>>();
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&mut read_monkeys(&input)));
    println!("Part 2: {}", part2(&mut read_monkeys(&input)));
    Ok(())
}

struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
    inspections: u64,
    divisor: u64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            items: Vec::<u64>::new(),
            operation: Box::new(move |worry| worry),
            test: Box::new(move |worry| worry as usize),
            inspections: 0,
            divisor: 0,
        }
    }

    fn inspect_throw(&mut self, item: u64, lcm: Option<u64>) -> (u64, usize) {
        self.inspections += 1;
        let worry = match lcm {
            Some(x) => (self.operation)(item) % x,
            None => (self.operation)(item) / 3,
        };
        (worry, (self.test)(worry))
    }
}

fn read_monkeys(input: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();
    let mut divisor = 1u64; 
    let mut m0 = 0usize;
    let mut m1 = 0usize;
    let mut count = 0usize;

    monkeys.push(Monkey::new());
    for line in input {
        if line.starts_with("Monkey") {
            monkeys[count].id = line.split_whitespace().collect::<Vec<&str>>()[1]
                .replace(":", "")
                .parse::<usize>()
                .unwrap();
        } else if line.starts_with("Starting items") {
            monkeys[count].items = line
                .replace("Starting items: ", "")
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
        } else if line.starts_with("Operation") {
            let op_string = line.replace("Operation: new = old ", "");
            let items = op_string.split(" ").collect::<Vec<&str>>();
            let value = match items[1] {
                "old" => {
                    monkeys[count].operation = Box::new(move |worry| worry * worry);
                    continue;
                }
                _ => items[1].parse::<u64>().unwrap(),
            };
            monkeys[count].operation = match items[0] {
                "+" => Box::new(move |worry| worry + value),
                "*" => Box::new(move |worry| worry * value),
                _ => Box::new(move |worry| worry),
            }
        } else if line.starts_with("Test") {
            let items = line.replace("Test: divisible by ", "");
            divisor = items.parse::<u64>().unwrap();
            monkeys[count].divisor = divisor;
        } else if line.starts_with("If true") {
            m0 = line
                .replace("If true: throw to monkey ", "")
                .trim()
                .parse::<usize>()
                .unwrap();
        } else if line.starts_with("If false") {
            m1 = line
                .replace("If false: throw to monkey ", "")
                .trim()
                .parse::<usize>()
                .unwrap();
            monkeys[count].test = Box::new(move |worry| {
                if worry % divisor == 0 {
                    return m0;
                } else {
                    return m1;
                }
            });
            monkeys.push(Monkey::new());
            count += 1;
        }
    }
    monkeys.pop();
    monkeys
}

fn part1(monkeys: &mut Vec<Monkey>) -> u64 {
    for _ in 0..NUMBER_OF_ROUNDS_1 {
        for index in 0..monkeys.len() {
            let mut monkey = monkeys.remove(index);
            while let Some(item) = monkey.items.pop() {
                let (updated_item, next_monkey) = monkey.inspect_throw(item, None);
                monkeys[next_monkey - {
                    if next_monkey > index {
                        1
                    } else {
                        0
                    }
                }]
                .items
                .push(updated_item);
            }
            monkeys.insert(index, monkey);
        }
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}

fn part2(monkeys: &mut Vec<Monkey>) -> u64 {
    let lcm = monkeys.iter().map(|m| m.divisor).fold(1, |acc, x| acc * x);
    for _ in 0..NUMBER_OF_ROUNDS_2 {
        for index in 0..monkeys.len() {
            let mut monkey = monkeys.remove(index);
            while let Some(item) = monkey.items.pop() {
                let (updated_item, next_monkey) = monkey.inspect_throw(item, Some(lcm));
                monkeys[next_monkey - {
                    if next_monkey > index {
                        1
                    } else {
                        0
                    }
                }]
                .items
                .push(updated_item);
            }
            monkeys.insert(index, monkey);
        }
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}
