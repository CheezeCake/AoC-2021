use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Range {
    start: isize,
    end: isize,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<&str> = s[2..].split("..").collect();
        assert_eq!(nums.len(), 2);
        Ok(Self {
            start: nums[0].parse::<isize>()?,
            end: nums[1].parse::<isize>()?,
        })
    }
}

#[derive(Clone, Debug)]
struct Cuboid {
    x_range: Range,
    y_range: Range,
    z_range: Range,
}

impl FromStr for Cuboid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<&str> = s.split(',').collect();
        assert_eq!(ranges.len(), 3);
        Ok(Self {
            x_range: ranges[0].parse::<Range>()?,
            y_range: ranges[1].parse::<Range>()?,
            z_range: ranges[2].parse::<Range>()?,
        })
    }
}

impl Cuboid {
    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        for (rs, ro) in [&self.x_range, &self.y_range, &self.z_range].iter().zip([
            &other.x_range,
            &other.y_range,
            &other.z_range,
        ]) {
            if rs.end < ro.start || ro.end < rs.start {
                return None;
            }
        }

        Some(Cuboid {
            x_range: Range {
                start: self.x_range.start.max(other.x_range.start),
                end: self.x_range.end.min(other.x_range.end),
            },
            y_range: Range {
                start: self.y_range.start.max(other.y_range.start),
                end: self.y_range.end.min(other.y_range.end),
            },
            z_range: Range {
                start: self.z_range.start.max(other.z_range.start),
                end: self.z_range.end.min(other.z_range.end),
            },
        })
    }

    fn volume(&self) -> isize {
        (self.x_range.end - self.x_range.start + 1)
            * (self.y_range.end - self.y_range.start + 1)
            * (self.z_range.end - self.z_range.start + 1)
    }
}

#[derive(Debug)]
struct RebootStep {
    on: bool,
    cuboid: Cuboid,
}

impl FromStr for RebootStep {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        assert_eq!(parts.len(), 2);

        let on = match parts[0] {
            "on" => true,
            "off" => false,
            _ => panic!("error parsing reboot step"),
        };

        let cuboid = parts[1].trim().parse::<Cuboid>()?;

        Ok(Self { on, cuboid })
    }
}

fn main() {
    let steps: Vec<RebootStep> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .parse::<RebootStep>()
                .expect("error parsing reboot step")
        })
        .collect();

    let mut reactor: Vec<(Cuboid, bool)> = Vec::new();

    for step in steps {
        let mut new_reactor = reactor.clone();
        for (cuboid, on) in &reactor {
            if let Some(inter) = cuboid.intersection(&step.cuboid) {
                new_reactor.push((
                    inter,
                    match (step.on, on) {
                        (true, true) => false,
                        (true, false) => true,
                        (false, true) => false,
                        (false, false) => true,
                    },
                ));
            }
        }
        if step.on {
            new_reactor.push((step.cuboid, step.on));
        }
        reactor = new_reactor;
    }

    let region = Cuboid {
        x_range: Range {
            start: -50,
            end: 50,
        },
        y_range: Range {
            start: -50,
            end: 50,
        },
        z_range: Range {
            start: -50,
            end: 50,
        },
    };

    println!(
        "part 1: {}",
        reactor
            .iter()
            .map(|(cuboid, on)| {
                cuboid
                    .intersection(&region)
                    .map_or(0, |inter| inter.volume() * if *on { 1 } else { -1 })
            })
            .sum::<isize>()
    );
    println!(
        "part 2: {}",
        reactor
            .iter()
            .map(|(cuboid, on)| { cuboid.volume() * if *on { 1 } else { -1 } })
            .sum::<isize>()
    );
}
