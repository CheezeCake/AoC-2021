use std::io;

fn fishes_after(timers: &[usize; 9], days: usize) -> usize {
    let mut timers = timers.clone();
    for _ in 0..days {
        timers.rotate_left(1);
        timers[6] += timers[8];
    }
    timers.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut timers = [0usize; 9];
    for n in input.split(',') {
        let n: usize = n.trim().parse().expect("error reading input");
        timers[n] += 1;
    }

    println!("part 1: {}", fishes_after(&timers, 80));
    println!("part 2: {}", fishes_after(&timers, 256));
}
