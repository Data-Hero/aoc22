use itertools::{EitherOrBoth::*, Itertools};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("Part 1: {}", part1(&mut input));
    println!("Part 2: {}", part2(&mut input));
    Ok(())
}

#[derive(Debug, Clone)]
enum Token {
    Number(u8),
    List(Vec<Token>),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Number(a), Token::Number(b)) => a == b,
            (Token::List(a), Token::List(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.clone();
        let b = other.clone();
        match (a, b) {
            (Token::Number(a), Token::Number(b)) => {
                if a < b {
                    return Ordering::Less;
                } else if a > b {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            }
            (Token::Number(a), Token::List(b)) => {
                return Token::List(vec![Token::Number(a.clone())]).cmp(&mut Token::List(b.clone()))
            }
            (Token::List(a), Token::Number(b)) => {
                return Token::List(a.clone()).cmp(&mut Token::List(vec![Token::Number(b.clone())]))
            }
            (Token::List(a), Token::List(b)) => {
                for pair in a.iter().zip_longest(b.iter()) {
                    match pair {
                        Both(l, r) => if l != r { return l.cmp(r); },
                        Left(_l) => {
                            return Ordering::Greater;
                        }
                        Right(_r) => {
                            return Ordering::Less;
                        }
                    }
                }
                return Ordering::Equal;
            }
        }
    }
}

impl Eq for Token {}

fn lex(line: &mut String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::<Token>::new();
    let mut number = String::new();
    while !line.is_empty() {
        let c = line.remove(0);
        match c {
            '0'..='9' => number.push(c),
            '[' => tokens.push(Token::List(lex(line))),
            ']' => {
                if !number.is_empty() {
                    tokens.push(Token::Number(number.parse::<u8>().unwrap()));
                }
                return tokens;
            }
            ',' => {
                if !number.is_empty() {
                    tokens.push(Token::Number(number.parse::<u8>().unwrap()));
                }
                number = String::new();
            }
            _ => {}
        }
    }
    tokens
}

fn part1(input: &mut Vec<String>) -> usize {
    let mut result = 0;
    let mut count = 0;
    for index in (0..input.len() - 1).step_by(3) {
        count += 1;
        let l1 = lex(&mut input[index].clone());
        let l2 = lex(&mut input[index + 1].clone());
        if l1.is_empty() {
            break;
        }
        if l2.is_empty() || l1 <= l2 {
            result += count;
        }
    }
    result
}

fn part2(input: &mut Vec<String>) -> usize {
    let dp1 = Token::List(vec![Token::List(vec![Token::Number(2)])]);
    let dp2 = Token::List(vec![Token::List(vec![Token::Number(6)])]);
    let mut lines = vec![dp1.clone(), dp2.clone()];
    for line in input {
        let l = lex(&mut line.clone());
        if l.is_empty() {
            continue;
        }
        lines.push(Token::List(l));
    }
    lines.sort();
    (lines.iter().position(|l| l == &dp1).unwrap() + 1) * (lines.iter().position(|l| l == &dp2).unwrap() + 1)
}
