use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() {
    let mut adacency_list: HashMap<String, Vec<String>> = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('-').collect();
        assert_eq!(parts.len(), 2);

        let a = parts[0].to_string();
        let b = parts[1].to_string();

        adacency_list
            .entry(a.clone())
            .or_insert(Vec::new())
            .push(b.clone());
        adacency_list.entry(b).or_insert(Vec::new()).push(a);
    }

    let mut path_count = 0;
    let mut stack: Vec<(String, HashSet<String>)> = Vec::new();

    let mut s = HashSet::new();
    s.insert("start".to_string());
    stack.push(("start".to_string(), s));

    while let Some((position, small_caves_visited)) = stack.pop() {
        if position == "end" {
            path_count += 1;
            continue;
        }
        for adjacent_cave in adacency_list.get(&position).unwrap() {
            if adjacent_cave.as_bytes()[0].is_ascii_uppercase() {
                stack.push((adjacent_cave.to_string(), small_caves_visited.clone()));
            } else if !small_caves_visited.contains(adjacent_cave) {
                let mut visited = small_caves_visited.clone();
                visited.insert(adjacent_cave.to_string());
                stack.push((adjacent_cave.to_string(), visited));
            }
        }
    }

    println!("part 1: {}", path_count);

    let mut path_count = 0;
    let mut stack: Vec<(String, HashSet<String>, bool)> = Vec::new();

    let mut s = HashSet::new();
    s.insert("start".to_string());
    stack.push(("start".to_string(), s, false));

    while let Some((position, small_caves_visited, small_visited_twice)) = stack.pop() {
        if position == "end" {
            path_count += 1;
            continue;
        }
        for adjacent_cave in adacency_list.get(&position).unwrap() {
            if adjacent_cave.as_bytes()[0].is_ascii_uppercase() {
                stack.push((
                    adjacent_cave.to_string(),
                    small_caves_visited.clone(),
                    small_visited_twice,
                ));
            } else if adjacent_cave != "start" {
                if !small_caves_visited.contains(adjacent_cave) {
                    let mut visited = small_caves_visited.clone();
                    visited.insert(adjacent_cave.to_string());
                    stack.push((adjacent_cave.to_string(), visited, small_visited_twice));
                } else if !small_visited_twice {
                    let mut visited = small_caves_visited.clone();
                    visited.insert(adjacent_cave.to_string());
                    stack.push((adjacent_cave.to_string(), visited, true));
                }
            }
        }
    }

    println!("part 2: {}", path_count);
}
