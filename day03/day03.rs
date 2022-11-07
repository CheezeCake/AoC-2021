use std::cmp::Ordering;
use std::io;
use std::io::BufRead;
use std::str;

fn count_bits(report: &[Vec<u8>], index: usize, valid: impl Fn(usize) -> bool) -> (usize, usize) {
    report
        .iter()
        .enumerate()
        .filter(|&(i, _)| valid(i))
        .map(|(_, num)| num)
        .fold((0, 0), |(zeros, ones), num| {
            if num[index] == b'0' {
                (zeros + 1, ones)
            } else {
                (zeros, ones + 1)
            }
        })
}

fn find_gamma_epsilon(
    nums: &[Vec<u8>],
    bits: usize,
    bit_criteria: impl Fn(usize, usize) -> u8,
) -> Vec<u8> {
    (0..bits)
        .map(|bit_index| {
            let (zeros, ones) = count_bits(nums, bit_index, |_| true);
            bit_criteria(zeros, ones)
        })
        .collect()
}

fn find_oxygen_co2(
    report: &[Vec<u8>],
    num_len: usize,
    bit_criteria: impl Fn(usize, usize) -> u8,
) -> Option<Vec<u8>> {
    let mut valid = vec![true; report.len()];
    let mut remaining = report.len();

    for index in 0..num_len {
        let (zeros, ones) = count_bits(report, index, |i| valid[i]);
        let bc = bit_criteria(zeros, ones);
        for (i, num) in report.iter().enumerate() {
            if valid[i] && num[index] != bc {
                valid[i] = false;
                remaining -= 1;
            }
        }
        if remaining == 1 {
            for (i, num) in report.iter().enumerate() {
                if valid[i] {
                    return Some(num.to_vec());
                }
            }
        }
    }

    None
}

fn most_common_bit(zeros: usize, ones: usize) -> u8 {
    match zeros.cmp(&ones) {
        Ordering::Less | Ordering::Equal => b'1',
        Ordering::Greater => b'0',
    }
}

fn least_common_bit(zeros: usize, ones: usize) -> u8 {
    match zeros.cmp(&ones) {
        Ordering::Less | Ordering::Equal => b'0',
        Ordering::Greater => b'1',
    }
}

fn bin2int(num: &[u8]) -> u32 {
    u32::from_str_radix(str::from_utf8(num).unwrap(), 2).unwrap()
}

fn main() {
    let report: Vec<Vec<u8>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();
    assert!(report.len() > 0);
    let num_len = report[0].len();

    let gamma_bin = find_gamma_epsilon(&report, num_len, most_common_bit);
    let gamma = bin2int(&gamma_bin);

    let epsilon_bin = find_gamma_epsilon(&report, num_len, least_common_bit);
    let epsilon = bin2int(&epsilon_bin);

    println!("part 1: {}", gamma * epsilon);

    let oxygen_bin = find_oxygen_co2(&report, num_len, most_common_bit)
        .expect("could not find oxygen generator rating");
    let oxygen = bin2int(&oxygen_bin);

    let co2_bin = find_oxygen_co2(&report, num_len, least_common_bit)
        .expect("could not find CO2 scrubber rating");
    let co2 = bin2int(&co2_bin);

    println!("part 2: {}", oxygen * co2);
}
