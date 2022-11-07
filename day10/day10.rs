use std::io;
use std::io::BufRead;

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut error_score = 0;
    let mut completion_scores: Vec<u64> = Vec::new();

    for line in lines {
        let mut stack = Vec::new();
        let mut corrupted = false;

        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                ')' | ']' | '}' | '>' => {
                    if let Some(expected) = stack.pop() {
                        if c != expected {
                            corrupted = true;
                            error_score += match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!(),
                            };
                            break;
                        }
                    } else {
                        panic!("unexpected extra closing character");
                    }
                }
                _ => panic!("invalid character"),
            }
        }

        if !corrupted && stack.len() > 0 {
            completion_scores.push(stack.iter().rev().fold(0, |score, expected| {
                score * 5
                    + match expected {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            }));
        }
    }

    println!("part 1: {}", error_score);

    completion_scores.sort();
    assert_eq!(completion_scores.len() % 2, 1);
    println!("part 2: {}", completion_scores[completion_scores.len() / 2]);
}
