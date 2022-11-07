use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::str;

fn step(
    formula: &HashMap<(u8, u8), usize>,
    rules: &HashMap<(u8, u8), u8>,
) -> HashMap<(u8, u8), usize> {
    let mut result = HashMap::new();

    for (pair, n) in formula {
        if let Some(&insertion) = rules.get(pair) {
            *result.entry((pair.0, insertion)).or_insert(0) += n;
            *result.entry((insertion, pair.1)).or_insert(0) += n;
        }
    }

    result
}

fn count_occurences(formula: &HashMap<(u8, u8), usize>) -> HashMap<u8, usize> {
    let mut counter = HashMap::new();
    for (&(a, b), n) in formula {
        *counter.entry(a).or_insert(0) += n;
        *counter.entry(b).or_insert(0) += n;
    }
    counter
}

fn most_common_element_count(counter: &HashMap<u8, usize>) -> usize {
    let occurences = counter.values().max().unwrap();
    if occurences % 2 == 0 {
        occurences / 2
    } else {
        occurences / 2 + 1
    }
}

fn least_common_element_count(counter: &HashMap<u8, usize>) -> usize {
    let occurences = counter.values().min().unwrap();
    if occurences % 2 == 0 {
        occurences / 2
    } else {
        occurences / 2 + 1
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error reading polymer template");
    let template = input.trim();

    let rules: HashMap<(u8, u8), u8> = io::stdin()
        .lock()
        .lines()
        .skip(1)
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(" -> ").collect();
            assert_eq!(parts.len(), 2);

            let pair = parts[0].as_bytes();
            assert_eq!(pair.len(), 2);
            let insertion = parts[1].as_bytes();
            assert_eq!(insertion.len(), 1);

            ((pair[0], pair[1]), insertion[0])
        })
        .collect();

    let mut formula = HashMap::new();
    for pair in template.bytes().zip(template.bytes().skip(1)) {
        *formula.entry(pair).or_insert(0) += 1;
    }

    for _ in 1..=10 {
        formula = step(&formula, &rules);
    }

    let counter = count_occurences(&formula);
    println!(
        "part 1: {}",
        most_common_element_count(&counter) - least_common_element_count(&counter)
    );

    for _ in 11..=40 {
        formula = step(&formula, &rules);
    }

    let counter = count_occurences(&formula);
    println!(
        "part 2: {}",
        most_common_element_count(&counter) - least_common_element_count(&counter)
    );
}
