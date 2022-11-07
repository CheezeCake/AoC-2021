use std::cmp::Ordering;
use std::fmt;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

enum Variable {
    W,
    X,
    Y,
    Z,
}

enum Argument {
    Variable(Variable),
    Immediate(i64),
}

enum Instruction {
    Inp(Variable),
    Add(Variable, Argument),
    Mul(Variable, Argument),
    Div(Variable, Argument),
    Mod(Variable, Argument),
    Eql(Variable, Argument),
}

enum InstructionErrorKind {
    Mnemonic,
    Variable,
    Immediate(ParseIntError),
    MissingArgs,
    TooManyArgs,
}

struct ParseInstructionError {
    kind: InstructionErrorKind,
}

impl fmt::Debug for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            InstructionErrorKind::Mnemonic => write!(f, "invalid mnemonic"),
            InstructionErrorKind::Variable => write!(f, "invalid variable"),
            InstructionErrorKind::Immediate(e) => write!(f, "invalid immediate value: {}", e),
            InstructionErrorKind::MissingArgs => write!(f, "missing arguments"),
            InstructionErrorKind::TooManyArgs => write!(f, "too many arguments"),
        }
    }
}

impl FromStr for Variable {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Variable::W),
            "x" => Ok(Variable::X),
            "y" => Ok(Variable::Y),
            "z" => Ok(Variable::Z),
            _ => Err(ParseInstructionError {
                kind: InstructionErrorKind::Variable,
            }),
        }
    }
}

impl FromStr for Argument {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Variable>() {
            Ok(v) => Ok(Argument::Variable(v)),
            Err(_) => match s.parse::<i64>() {
                Ok(val) => Ok(Argument::Immediate(val)),
                Err(e) => Err(ParseInstructionError {
                    kind: InstructionErrorKind::Immediate(e),
                }),
            },
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        match tokens[0] {
            "inp" => match tokens.len().cmp(&2) {
                Ordering::Less => Err(ParseInstructionError {
                    kind: InstructionErrorKind::MissingArgs,
                }),
                Ordering::Equal => Ok(Instruction::Inp(tokens[1].parse()?)),
                Ordering::Greater => Err(ParseInstructionError {
                    kind: InstructionErrorKind::TooManyArgs,
                }),
            },

            "add" | "mul" | "div" | "mod" | "eql" => match tokens.len().cmp(&3) {
                Ordering::Less => Err(ParseInstructionError {
                    kind: InstructionErrorKind::MissingArgs,
                }),
                Ordering::Equal => match tokens[0] {
                    "add" => Ok(Instruction::Add(tokens[1].parse()?, tokens[2].parse()?)),
                    "mul" => Ok(Instruction::Mul(tokens[1].parse()?, tokens[2].parse()?)),
                    "div" => Ok(Instruction::Div(tokens[1].parse()?, tokens[2].parse()?)),
                    "mod" => Ok(Instruction::Mod(tokens[1].parse()?, tokens[2].parse()?)),
                    "eql" => Ok(Instruction::Eql(tokens[1].parse()?, tokens[2].parse()?)),
                    _ => unreachable!(),
                },
                Ordering::Greater => Err(ParseInstructionError {
                    kind: InstructionErrorKind::TooManyArgs,
                }),
            },

            _ => {
                return Err(ParseInstructionError {
                    kind: InstructionErrorKind::Mnemonic,
                })
            }
        }
    }
}

fn solve(vals: &[(i64, i64, i64)]) -> (i64, i64) {
    let mut stack = Vec::new();
    let mut min_val = [0; 14];
    let mut max_val = [0; 14];

    for i in 0..14 {
        let (ai, bi, _) = vals[i];
        if ai == 1 {
            stack.push(i)
        } else {
            let j = stack.pop().unwrap();
            let (_, _, cj) = vals[j];

            let m = (9 + cj).min(9 - bi);
            max_val[j] = m - cj;
            max_val[i] = m + bi;

            let m = (1 + cj).max(1 - bi);
            min_val[j] = m - cj;
            min_val[i] = m + bi;
        }
    }

    (
        min_val.iter().fold(0, |acc, x| acc * 10 + x),
        max_val.iter().fold(0, |acc, x| acc * 10 + x),
    )
}

fn main() {
    let program: Vec<Instruction> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("error reading input")
                .parse()
                .expect("error parsing input")
        })
        .collect();

    assert_eq!(program.len() % 18, 0);

    let mut vals: Vec<(i64, i64, i64)> = Vec::new();

    for p_start in (0..program.len()).step_by(18) {
        let a = match &program[p_start + 4] {
            Instruction::Div(_, arg) => match arg {
                Argument::Immediate(val) => val,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        let b = match &program[p_start + 5] {
            Instruction::Add(_, arg) => match arg {
                Argument::Immediate(val) => val,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        let c = match &program[p_start + 15] {
            Instruction::Add(_, arg) => match arg {
                Argument::Immediate(val) => val,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        vals.push((*a, *b, *c));
    }

    assert_eq!(vals.len(), 14);

    let (min, max) = solve(&vals);
    println!("part 1: {}", max);
    println!("part 2: {}", min);
}
