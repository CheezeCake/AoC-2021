use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;
use std::io::BufRead;

struct Cavern {
    map: Vec<Vec<usize>>,
}

impl Cavern {
    fn get(&self, y: usize, x: usize) -> usize {
        let tile_y = y / self.map.len();
        let y = y % self.map.len();
        let tile_x = x / self.map.len();
        let x = x % self.map.len();

        let risk = self.map[y][x] + tile_y + tile_x;
        if risk > 9 {
            risk % 9
        } else {
            risk
        }
    }
}

fn lowest_total_risk(cavern: &Cavern, len: usize) -> usize {
    let mut dist: Vec<Vec<usize>> = (0..len)
        .map(|_| (0..len).map(|_| usize::MAX).collect())
        .collect();
    dist[0][0] = 0;

    let mut q = BinaryHeap::new();
    q.push(Reverse((dist[0][0], 0, 0)));

    while let Some(Reverse((cost, y, x))) = q.pop() {
        if y == len - 1 && x == len - 1 {
            return cost;
        }

        if cost > dist[y][x] {
            continue;
        }

        for (dy, dx) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
            let next_x = (x as isize) + dx;
            let next_y = (y as isize) + dy;
            if next_x >= 0 && (next_x as usize) < len && next_y >= 0 && (next_y as usize) < len {
                let next_x = next_x as usize;
                let next_y = next_y as usize;
                let next_cost = cost + cavern.get(next_y, next_x);

                if next_cost < dist[next_y][next_x] {
                    q.push(Reverse((next_cost, next_y, next_x)));
                    dist[next_y][next_x] = next_cost;
                }
            }
        }
    }

    unreachable!()
}

fn main() {
    let map: Vec<Vec<usize>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .trim()
                .bytes()
                .map(|b| (b - b'0') as usize)
                .collect()
        })
        .collect();

    let cavern = Cavern { map };

    println!("part 1: {}", lowest_total_risk(&cavern, cavern.map.len()));
    println!(
        "part 2: {}",
        lowest_total_risk(&cavern, cavern.map.len() * 5)
    );
}
