use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn within_bounds(y: isize, x: isize, grid: &Vec<Vec<u32>>) -> bool {
    y >= 0 && (y as usize) < grid.len() && x >= 0 && (x as usize) < grid[y as usize].len()
}

fn step(grid: &mut Vec<Vec<u32>>) -> usize {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x] += 1;
        }
    }

    let mut flashed = HashSet::new();

    loop {
        let mut flashing = Vec::new();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] > 9 && !flashed.contains(&(y, x)) {
                    flashing.push((y, x));
                }
            }
        }

        if flashing.len() == 0 {
            return flashed.len();
        }

        for &(y, x) in &flashing {
            flashed.insert((y, x));

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let y = (y as isize) - dy;
                    let x = (x as isize) - dx;
                    if within_bounds(y, x, grid) {
                        grid[y as usize][x as usize] += 1;
                    }
                }
            }
        }

        for &(y, x) in &flashed {
            grid[y][x] = 0;
        }
    }
}

fn main() {
    let mut grid: Vec<Vec<u32>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().bytes().map(|b| (b - b'0') as u32).collect())
        .collect();

    let len = grid.len() * grid[0].len();
    let mut total_flashes = 0;
    let mut all_flashed = None;

    for s in 1..=100 {
        let flashes = step(&mut grid);
        total_flashes += flashes;
        if flashes == len {
            all_flashed = Some(s);
        }
    }

    println!("part 1: {}", total_flashes);

    if all_flashed.is_none() {
        for s in 101.. {
            let flashes = step(&mut grid);
            if flashes == len {
                all_flashed = Some(s);
                break;
            }
        }
    }

    println!("part 2: {}", all_flashed.unwrap());
}
