use std::collections::HashSet;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum PointParseError {
    Malformed,
    ParseIntError(ParseIntError),
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 2 {
            return Err(Self::Err::Malformed);
        }

        let x = coords[0]
            .parse()
            .or_else(|e| Err(Self::Err::ParseIntError(e)))?;
        let y = coords[1]
            .parse()
            .or_else(|e| Err(Self::Err::ParseIntError(e)))?;

        Ok(Self { x, y })
    }
}

enum Axis {
    X,
    Y,
}

struct Instruction {
    axis: Axis,
    coord: usize,
}

#[derive(Debug)]
enum InstructionParseError {
    Malformed,
    InvalidAxis(char),
    ParseIntError(ParseIntError),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split('=').collect();
        if parts.len() != 2 {
            return Err(Self::Err::Malformed);
        }

        let axis = match parts[0].chars().last() {
            Some('x') => Axis::X,
            Some('y') => Axis::Y,
            Some(c) => return Err(Self::Err::InvalidAxis(c)),
            None => return Err(Self::Err::Malformed),
        };
        let coord = parts[1]
            .parse()
            .or_else(|e| Err(Self::Err::ParseIntError(e)))?;

        Ok(Self { axis, coord })
    }
}

struct Page {
    dots: HashSet<Point>,
    height: usize,
    width: usize,
}

impl Page {
    fn new(dots: HashSet<Point>) -> Self {
        let width = dots.iter().map(|p| p.x).max().unwrap() + 1;
        let height = dots.iter().map(|p| p.y).max().unwrap() + 1;
        Self {
            dots,
            height,
            width,
        }
    }

    fn within_page(&self, p: &Point) -> bool {
        p.x < self.width && p.y < self.height
    }

    fn fold_up(&mut self, along: usize) {
        let bottom_part: Vec<Point> = self
            .dots
            .iter()
            .filter(|p| self.within_page(p) && p.y > along)
            .copied()
            .collect();
        for p in bottom_part {
            self.dots.insert(Point {
                x: p.x,
                y: along - (p.y - along),
            });
        }
        self.height = along;
    }

    fn fold_left(&mut self, along: usize) {
        let right_part: Vec<Point> = self
            .dots
            .iter()
            .filter(|p| self.within_page(p) && p.x > along)
            .copied()
            .collect();
        for p in right_part {
            self.dots.insert(Point {
                x: along - (p.x - along),
                y: p.y,
            });
        }
        self.width = along;
    }

    fn visible_dots(&self) -> usize {
        self.dots.iter().filter(|p| self.within_page(p)).count()
    }
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.dots.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            if y != self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn apply_instruction(page: &mut Page, instruction: &Instruction) {
    match instruction.axis {
        Axis::X => page.fold_left(instruction.coord),
        Axis::Y => page.fold_up(instruction.coord),
    }
}

fn main() {
    let mut dots: HashSet<Point> = HashSet::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.len() <= 1 {
            break;
        }
        dots.insert(input.trim().parse().expect("error parsing dot in input"));
    }

    let mut instructions: Vec<Instruction> = Vec::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => instructions.push(input.parse().expect("error parsing instruction in input")),
            Err(e) => panic!("{}", e),
        }
    }

    let mut page = Page::new(dots);

    apply_instruction(&mut page, &instructions[0]);

    println!("part 1: {}", page.visible_dots());

    for instruction in &instructions[1..] {
        apply_instruction(&mut page, instruction);
    }

    println!("part 2:\n{}", page);
}
