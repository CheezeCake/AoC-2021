use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

fn change_orientation(p: (isize, isize, isize), orientation: usize) -> (isize, isize, isize) {
    let (x, y, z) = p;
    match orientation {
        0 => (x, y, z),
        1 => (-y, x, z),
        2 => (-x, -y, z),
        3 => (y, x, z),

        4 => (-x, y, -z),
        5 => (-y, -x, -z),
        6 => (x, -y, -z),
        7 => (y, x, -z),

        8 => (-z, y, x),
        9 => (-y, -z, x),
        10 => (z, -z, x),
        11 => (y, z, x),

        12 => (z, y, -x),
        13 => (-y, z, -x),
        14 => (-z, -y, -x),
        15 => (y, -z, -x),

        16 => (x, -z, y),
        17 => (z, x, y),
        18 => (-x, z, y),
        19 => (-z, -x, y),

        20 => (x, z, -y),
        21 => (-z, x, -y),
        22 => (-x, -z, -y),
        23 => (z, -x, -y),

        _ => panic!("invalid orientation"),
    }
}

fn intersection_path_to(
    scanner: usize,
    to: usize,
    intersections: &HashMap<usize, HashMap<usize, ((isize, isize, isize), usize)>>,
) -> Option<Vec<usize>> {
    let mut q = VecDeque::new();
    q.push_back((scanner, Vec::new(), HashSet::new()));

    while let Some((s, path, visited)) = q.pop_front() {
        if s == to {
            return Some(path);
        }

        for (&next, _) in intersections.get(&s).unwrap_or(&HashMap::new()) {
            if !visited.contains(&next) {
                let mut p = path.clone();
                let mut v = visited.clone();
                p.push(next);
                v.insert(next);
                q.push_back((next, p, v));
            }
        }
    }

    None
}

fn main() {
    let mut reports: Vec<HashSet<(isize, isize, isize)>> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.len() == 0 {
            break;
        }

        let mut beacons = HashSet::new();

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.len() <= 1 {
                reports.push(beacons);
                break;
            }

            let coords: Vec<&str> = input.trim().split(',').collect();
            assert_eq!(coords.len(), 3);
            let x = coords[0].parse().unwrap();
            let y = coords[1].parse().unwrap();
            let z = coords[2].parse().unwrap();

            beacons.insert((x, y, z));
        }
    }

    let mut inter = HashMap::new();
    inter
        .entry(0)
        .or_insert(HashMap::new())
        .insert(0, ((0, 0, 0), 0));

    for scanner1 in 0..reports.len() {
        for scanner2 in 0..reports.len() {
            if scanner1 == scanner2 {
                continue;
            }
            for orientation in 0..24 {
                let mut cnt = HashMap::new();

                for &beacon1 in &reports[scanner1] {
                    for &beacon2 in &reports[scanner2] {
                        let beacon2 = change_orientation(beacon2, orientation);
                        let delta = (
                            beacon1.0 - beacon2.0,
                            beacon1.1 - beacon2.1,
                            beacon1.2 - beacon2.2,
                        );
                        *cnt.entry(delta).or_insert(0) += 1;
                    }
                }

                for (delta, n) in cnt {
                    if n >= 12 {
                        inter
                            .entry(scanner2)
                            .or_insert(HashMap::new())
                            .insert(scanner1, (delta, orientation));
                    }
                }
            }
        }
    }

    let mut world = reports[0].clone();
    let mut scanner_delta_from_0 = Vec::new();
    scanner_delta_from_0.resize(reports.len(), (0, 0, 0));

    for scanner in 1..reports.len() {
        if let Some(inter_path) = intersection_path_to(scanner, 0, &inter) {
            let mut beacons: Vec<(isize, isize, isize)> =
                reports[scanner].iter().copied().collect();
            let mut scanner_delta = (0, 0, 0);
            let mut prev = scanner;

            for next in inter_path {
                let &(delta, orientation) = inter.get(&prev).unwrap().get(&next).unwrap();
                scanner_delta = change_orientation(scanner_delta, orientation);
                scanner_delta = (
                    scanner_delta.0 + delta.0,
                    scanner_delta.1 + delta.1,
                    scanner_delta.2 + delta.2,
                );
                scanner_delta_from_0[scanner] = scanner_delta;

                for b in beacons.iter_mut() {
                    *b = change_orientation(*b, orientation);
                    *b = (b.0 + delta.0, b.1 + delta.1, b.2 + delta.2);
                }
                prev = next;
            }

            for b in beacons {
                world.insert(b);
            }
        } else {
            panic!("no intersection path from {} to {}", scanner, 0);
        }
    }

    println!("part 1: {}", world.len());

    let mut max_distance = 0;
    for scanner1 in 0..reports.len() {
        for scanner2 in (scanner1 + 1)..reports.len() {
            let scanner1_pos = scanner_delta_from_0[scanner1];
            let scanner2_pos = scanner_delta_from_0[scanner2];
            max_distance = max_distance.max(
                scanner1_pos.0.abs_diff(scanner2_pos.0)
                    + scanner1_pos.1.abs_diff(scanner2_pos.1)
                    + scanner1_pos.2.abs_diff(scanner2_pos.2),
            );
        }
    }
    println!("part 2: {}", max_distance);
}
