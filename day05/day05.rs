use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        assert_eq!(coords.len(), 2);

        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;

        Ok(Point { x, y })
    }
}

struct Segment {
    start: Point,
    end: Point,
}

impl FromStr for Segment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.split(" -> ").collect();
        assert_eq!(points.len(), 2);

        let start = points[0].parse::<Point>()?;
        let end = points[1].parse::<Point>()?;

        Ok(Segment { start, end })
    }
}

fn count_intersections(
    segments: &[Segment],
    direction: impl Fn(&Point, &Point) -> Option<(i32, i32)>,
) -> usize {
    let mut points_cnt = HashMap::new();

    for segment in segments {
        if let Some((dx, dy)) = direction(&segment.start, &segment.end) {
            let mut p = segment.start;
            let cnt = points_cnt.entry(p).or_insert(0);
            *cnt += 1;

            while p != segment.end {
                p.x += dx;
                p.y += dy;

                let cnt = points_cnt.entry(p).or_insert(0);
                *cnt += 1;
            }
        }
    }

    points_cnt.values().filter(|&cnt| *cnt > 1).count()
}

fn main() {
    let segments: Vec<Segment> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().expect("error parsing input"))
        .collect();

    println!(
        "part 1: {}",
        count_intersections(&segments, |start, end| {
            match (start.x.cmp(&end.x), start.y.cmp(&end.y)) {
                (Ordering::Less, Ordering::Equal) => Some((1, 0)),
                (Ordering::Equal, Ordering::Less) => Some((0, 1)),
                (Ordering::Equal, Ordering::Equal) => Some((0, 0)),
                (Ordering::Equal, Ordering::Greater) => Some((0, -1)),
                (Ordering::Greater, Ordering::Equal) => Some((-1, 0)),
                _ => None,
            }
        })
    );

    println!(
        "part 2: {}",
        count_intersections(&segments, |start, end| {
            match (start.x.cmp(&end.x), start.y.cmp(&end.y)) {
                (Ordering::Less, Ordering::Less) => Some((1, 1)),
                (Ordering::Less, Ordering::Equal) => Some((1, 0)),
                (Ordering::Less, Ordering::Greater) => Some((1, -1)),

                (Ordering::Equal, Ordering::Less) => Some((0, 1)),
                (Ordering::Equal, Ordering::Equal) => Some((0, 0)),
                (Ordering::Equal, Ordering::Greater) => Some((0, -1)),

                (Ordering::Greater, Ordering::Less) => Some((-1, 1)),
                (Ordering::Greater, Ordering::Equal) => Some((-1, 0)),
                (Ordering::Greater, Ordering::Greater) => Some((-1, -1)),
            }
        })
    );
}
