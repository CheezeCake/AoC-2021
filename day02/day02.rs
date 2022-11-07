use std::io;
use std::io::BufRead;

struct Command(String, u32);

struct Submarine {
    depth: i32,
    position: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Self {
        Self {
            depth: 0,
            position: 0,
            aim: 0,
        }
    }
}

fn main() {
    let commands: Vec<(String, i32)> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(' ').collect();
            assert!(parts.len() == 2);
            (parts[0].to_string(), parts[1].parse().unwrap())
        })
        .collect();

    let mut sub = Submarine::new();
    for (instr, value) in &commands {
        match instr.as_str() {
            "forward" => sub.position += value,
            "down" => sub.depth += value,
            "up" => sub.depth -= value,
            _ => panic!("invalid command"),
        }
    }
    println!("part 1: {}", sub.position * sub.depth);

    let mut sub = Submarine::new();
    for (instr, value) in &commands {
        match instr.as_str() {
            "forward" => {
                sub.position += value;
                sub.depth += sub.aim * value;
            }
            "down" => sub.aim += value,
            "up" => sub.aim -= value,
            _ => panic!("invalid command"),
        }
    }
    println!("part 2: {}", sub.position * sub.depth);
}
