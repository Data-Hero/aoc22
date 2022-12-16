use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::{Itertools, EitherOrBoth::*};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut input = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    println!("Input: {:?}", input);
    println!("Part 1: {}", part1(&mut input));
    println!("Part 2: {}", part2(&mut input));
    Ok(())
}

#[derive(Debug, Clone)]
enum Token {
    Number(u8),
    List(Vec<Token>),
}

#[derive(Debug, PartialEq)]
enum HT {
    F,
    NF,
    T,
}

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

fn compare(a: &mut Token, b: &mut Token) -> HT {
    match (a, b) {
        (Token::Number(a), Token::Number(b)) => {
            if a < b {
                return HT::T;
            } else if a > b {
                return HT::F;
            } else {
                return HT::NF;
            }
        }
        (Token::Number(a), Token::List(b)) => {
            return compare(
                &mut Token::List(vec![Token::Number(a.clone())]),
                &mut Token::List(b.clone()),
            )
        }
        (Token::List(a), Token::Number(b)) => {
            return compare(
                &mut Token::List(a.clone()),
                &mut Token::List(vec![Token::Number(b.clone())]),
            )
        }
        (Token::List(a), Token::List(b)) => {
            for pair in a.iter().zip_longest(b.iter()) {
                match pair {
                    Both(l, r) => match compare(&mut l.clone(), &mut r.clone()) {
                        HT::T => return HT::T,
                        HT::F => return HT::F,
                        HT::NF => {}
                    },
                    Left(_l) => {
                        return HT::F;
                    }
                    Right(_r) => {
                        return HT::T;
                    }
                }
            }
            return HT::NF;
        }
    }
}

fn part1(input: &mut Vec<String>) -> usize {
    let mut result = 0;
    let mut count = 0;
    for index in (0..input.len() - 1).step_by(3) {
        count += 1;
        let mut l1 = lex(&mut input[index]);
        let mut l2 = lex(&mut input[index + 1]);
        if l1.is_empty() {
            break;
        }

        let a = compare(&mut l1[0], &mut l2[0]);
        if l2.is_empty() || a == HT::T {
            result += count;
        }
    }
    result
}

fn part2(input: &Vec<String>) -> usize {
    0
}
