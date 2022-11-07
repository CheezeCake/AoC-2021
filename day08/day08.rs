use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct SignalPattern {
    segments: [bool; 7],
}

impl SignalPattern {
    fn active_segments(&self) -> usize {
        self.segments.iter().filter(|&&segment| segment).count()
    }

    fn intersection_len(&self, other: &Self) -> usize {
        self.segments
            .iter()
            .zip(other.segments.iter())
            .filter(|&(&s1, &s2)| s1 && s2)
            .count()
    }
}

#[derive(Debug)]
struct ParseSignalPatterError;

impl FromStr for SignalPattern {
    type Err = ParseSignalPatterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 || s.len() > 7 {
            return Err(Self::Err {});
        }

        let mut segments = [false; 7];
        for c in s.bytes() {
            match c {
                b'a'..=b'g' => segments[(c - b'a') as usize] = true,
                _ => return Err(Self::Err {}),
            }
        }

        Ok(SignalPattern { segments })
    }
}

#[derive(Debug)]
struct Display {
    signals: [SignalPattern; 10],
    output_values: [SignalPattern; 4],
}

#[derive(Debug)]
struct ParseDisplayError;

impl FromStr for Display {
    type Err = ParseDisplayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<Vec<SignalPattern>> = s
            .split(" | ")
            .map(|patterns| {
                patterns
                    .split(' ')
                    .map(|pattern| pattern.parse().unwrap())
                    .collect()
            })
            .collect();
        if parts.len() == 2 && parts[0].len() == 10 && parts[1].len() == 4 {
            Ok(Display {
                signals: parts[0].clone().try_into().unwrap(),
                output_values: parts[1].clone().try_into().unwrap(),
            })
        } else {
            Err(Self::Err {})
        }
    }
}

fn find_unique(
    patterns: &[SignalPattern],
    constraints: impl Fn(&SignalPattern) -> bool,
) -> Option<SignalPattern> {
    let patterns: Vec<&SignalPattern> = patterns
        .iter()
        .filter(|pattern| constraints(pattern))
        .collect();
    if patterns.len() == 1 {
        Some(*patterns[0])
    } else {
        None
    }
}

impl Display {
    fn decode_signals(&self) -> Option<HashMap<SignalPattern, u32>> {
        let one = find_unique(&self.signals, |signal| signal.active_segments() == 2)?;
        let four = find_unique(&self.signals, |signal| signal.active_segments() == 4)?;
        let seven = find_unique(&self.signals, |signal| signal.active_segments() == 3)?;
        let eight = find_unique(&self.signals, |signal| signal.active_segments() == 7)?;

        let mut decoded = HashMap::new();

        decoded.insert(one, 1);
        decoded.insert(four, 4);
        decoded.insert(seven, 7);
        decoded.insert(eight, 8);

        for (value, (len, int_one, int_four, int_seven)) in [
            (0, (6, 2, 3, 3)),
            (2, (5, 1, 2, 2)),
            (3, (5, 2, 3, 3)),
            (5, (5, 1, 3, 2)),
            (6, (6, 1, 3, 2)),
            (9, (6, 2, 4, 3)),
        ] {
            decoded.insert(
                find_unique(&self.signals, |signal| {
                    signal.active_segments() == len
                        && signal.intersection_len(&one) == int_one
                        && signal.intersection_len(&four) == int_four
                        && signal.intersection_len(&seven) == int_seven
                })?,
                value,
            );
        }

        return Some(decoded);
    }

    fn decode_ouput_values(&self) -> Option<[u32; 4]> {
        let decoded = self.decode_signals()?;
        Some([
            *decoded.get(&self.output_values[0])?,
            *decoded.get(&self.output_values[1])?,
            *decoded.get(&self.output_values[2])?,
            *decoded.get(&self.output_values[3])?,
        ])
    }
}

fn main() {
    let displays: Vec<Display> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().expect("error parsing input"))
        .collect();

    println!(
        "part 1: {}",
        displays
            .iter()
            .map(|display| {
                display
                    .output_values
                    .iter()
                    .filter(|value| match value.active_segments() {
                        2 | 3 | 4 | 7 => true,
                        _ => false,
                    })
                    .count()
            })
            .sum::<usize>()
    );

    println!(
        "part 2: {}",
        displays
            .iter()
            .map(|display| display
                .decode_ouput_values()
                .expect("could not decode output values")
                .iter()
                .fold(0, |acc, value| acc * 10 + value))
            .sum::<u32>()
    );
}
