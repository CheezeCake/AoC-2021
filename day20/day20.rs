use std::collections::HashMap;
use std::io;

type Image = HashMap<(isize, isize), bool>;

fn enhance(image: &Image, algorithm: &[u8], step: usize) -> Image {
    let min_y = *image.keys().map(|(y, _)| y).min().unwrap();
    let min_x = *image.keys().map(|(_, x)| x).min().unwrap();
    let max_y = *image.keys().map(|(y, _)| y).max().unwrap();
    let max_x = *image.keys().map(|(_, x)| x).max().unwrap();

    let mut enhanced = HashMap::new();

    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let mut t = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let ny = y + dy;
                    let nx = x + dx;
                    t *= 2;

                    if let Some(&v) = image.get(&(ny, nx)) {
                        t += if v { 1 } else { 0 };
                    } else if algorithm[0] == b'#' {
                        t += step % 2;
                    }
                }
            }

            enhanced.insert((y, x), algorithm[t] == b'#');
        }
    }

    enhanced
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let algo = input.trim().as_bytes().to_vec();
    assert_eq!(algo.len(), 512);

    let mut image: HashMap<(isize, isize), bool> = HashMap::new();
    let mut line = 0;

    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(1) => continue,
            Ok(_) => {
                for (column, c) in input.trim().bytes().enumerate() {
                    image.insert((line, column as isize), c == b'#');
                }
            }
            Err(e) => panic!("{}", e),
        }

        line += 1;
    }

    for step in 0..2 {
        image = enhance(&image, &algo, step);
    }
    println!("part 1: {}", image.values().filter(|&&v| v).count());

    for step in 2..50 {
        image = enhance(&image, &algo, step);
    }
    println!("part 2: {}", image.values().filter(|&&v| v).count());
}
