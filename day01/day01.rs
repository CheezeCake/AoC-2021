use std::io;
use std::io::BufRead;

fn main() {
    let depths: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .map(|depth| depth.unwrap().parse().unwrap())
        .collect();

    println!(
        "part 1: {}",
        depths
            .iter()
            .zip(depths.iter().skip(1))
            .filter(|(prev, cur)| prev < cur)
            .count()
    );

    let mut n = 0usize;
    let mut s: u32 = depths.iter().take(3).sum();
    for i in 3..depths.len() {
        let s1 = s;
        let s2 = s - depths[i - 3] + depths[i];
        if s1 < s2 {
            n += 1;
        }
        s = s2;
    }
    println!("part 2: {}", n);
}
