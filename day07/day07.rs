use std::io;

fn fuel_usage_sum(to: u32, from: &[u32], fuel_usage: impl Fn(u32, u32) -> u32) -> u32 {
    from.iter().map(|&from| fuel_usage(from, to)).sum()
}

fn solve(positions: &[u32], fuel_usage: impl Fn(u32, u32) -> u32) -> u32 {
    let mut low = *positions.iter().min().unwrap();
    let mut hi = *positions.iter().max().unwrap();

    while low < hi {
        let mid = (low + hi + 1) / 2;
        let low_cost = fuel_usage_sum(low, &positions, &fuel_usage);
        let hi_cost = fuel_usage_sum(hi, &positions, &fuel_usage);
        if low_cost < hi_cost {
            hi = mid - 1;
        } else if hi_cost < low_cost {
            low = mid;
        } else {
            hi -= 1;
        }
    }

    fuel_usage_sum(low, &positions, fuel_usage)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let crabs_positions: Vec<u32> = input
        .split(',')
        .map(|n| n.trim().parse().expect("error reading input"))
        .collect();

    println!(
        "part 1: {}",
        solve(&crabs_positions, |from, to| to.abs_diff(from))
    );
    println!(
        "part 2: {}",
        solve(&crabs_positions, |from, to| {
            let diff = to.abs_diff(from);
            (diff * (diff + 1)) / 2
        })
    );
}
