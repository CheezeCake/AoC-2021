use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn within_bounds(y: isize, x: isize, heightmap: &[Vec<u32>]) -> bool {
    y >= 0 && (y as usize) < heightmap.len() && x >= 0 && (x as usize) < heightmap[y as usize].len()
}

fn find_low_points(heightmap: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            let h = heightmap[y][x];
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            if directions
                .iter()
                .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
                .filter(|&(y, x)| within_bounds(y, x, &heightmap))
                .all(|(y, x)| h < heightmap[y as usize][x as usize])
            {
                low_points.push((y, x));
            }
        }
    }

    return low_points;
}

fn basin_size(
    y: usize,
    x: usize,
    heightmap: &[Vec<u32>],
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let h = heightmap[y][x];
    let mut size = 1;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (dy, dx) in directions {
        let y = y as isize + dy;
        let x = x as isize + dx;

        if within_bounds(y, x, heightmap) {
            let y = y as usize;
            let x = x as usize;
            if !visited.contains(&(y, x)) && heightmap[y][x] > h && heightmap[y][x] != 9 {
                visited.insert((y, x));
                size += basin_size(y, x, heightmap, visited);
            }
        }
    }

    return size;
}

fn main() {
    let heightmap: Vec<Vec<u32>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .trim()
                .bytes()
                .map(|c| (c - b'0') as u32)
                .collect()
        })
        .collect();

    let low_points = find_low_points(&heightmap);

    println!(
        "part 1: {}",
        low_points
            .iter()
            .map(|&(y, x)| heightmap[y][x] + 1)
            .sum::<u32>()
    );

    let mut basins: Vec<usize> = low_points
        .iter()
        .map(|&(y, x)| basin_size(y, x, &heightmap, &mut HashSet::new()))
        .collect();
    basins.sort();

    println!("part 2: {}", basins.iter().rev().take(3).product::<usize>());
}
