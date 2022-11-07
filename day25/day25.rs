use std::io;
use std::io::BufRead;

fn main() {
    let mut map: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("error reading input").chars().collect())
        .collect();

    let height = map.len();
    let width = map[0].len();

    for step in 1.. {
        let mut moved = false;

        for y in 0..height {
            let mut x = 0;
            let mut clear_first = false;

            while x < width {
                if map[y][x] == '>' {
                    let nx = (x + 1) % width;
                    if map[y][nx] == '.' {
                        if x > 0 {
                            map[y][x] = '.';
                        } else {
                            clear_first = true;
                        }
                        map[y][nx] = '>';
                        moved = true;
                        x += 2;
                    } else {
                        x += 1;
                    }
                } else {
                    x += 1;
                }
            }

            if clear_first {
                map[y][0] = '.';
            }
        }

        for x in 0..width {
            let mut y = 0;
            let mut clear_first = false;

            while y < height {
                if map[y][x] == 'v' {
                    let ny = (y + 1) % height;
                    if map[ny][x] == '.' {
                        if y > 0 {
                            map[y][x] = '.';
                        } else {
                            clear_first = true;
                        }

                        map[ny][x] = 'v';
                        moved = true;
                        y += 2;
                    } else {
                        y += 1;
                    }
                } else {
                    y += 1;
                }
            }

            if clear_first {
                map[0][x] = '.';
            }
        }

        if !moved {
            println!("part 1: {}", step);
            break;
        }
    }
}
