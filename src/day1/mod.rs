use std::collections::HashMap;
use std::fs;
use std::process;

// Solves Advent of Code 2024 day 1
pub fn solve() {
    // Part 1

    let input_path = format!("{}/src/day1/input.txt", env!("CARGO_MANIFEST_DIR"));
    let contents = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Problem parsing input file: {err}");
        process::exit(1);
    });

    let mut left_sorted: Vec<u32> = vec![];
    let mut right_sorted: Vec<u32> = vec![];

    // Stores how many times each number appears in the right list
    let mut right_counts: HashMap<u32, u32> = HashMap::new();

    for (i, line) in contents.split('\n').enumerate() {
        let mut location_ids = line.split_whitespace();

        let left_str: &str = match location_ids.next() {
            Some(s) => s,
            None => {
                println!("missing left number on line {}", i);
                continue;
            }
        };

        let left: u32 = left_str.parse().expect("Not a valid number");
        if left_sorted.is_empty() {
            left_sorted.push(left);
        } else {
            match left_sorted.binary_search(&left) {
                Ok(i) => left_sorted.insert(i, left),
                Err(i) => left_sorted.insert(i, left),
            };
        }

        let right_str: &str = match location_ids.next() {
            Some(s) => s,
            None => {
                println!("missing right number on line {}", i);
                continue;
            }
        };

        let right: u32 = right_str.parse().expect("Not a valid number");
        let count = right_counts.get_mut(&right);
        match count {
            Some(count) => {
                *count += 1;
            }
            None => {
                right_counts.insert(right, 1);
                ()
            }
        };

        if right_sorted.is_empty() {
            right_sorted.push(right);
        } else {
            match right_sorted.binary_search(&right) {
                Ok(i) => right_sorted.insert(i, right),
                Err(i) => right_sorted.insert(i, right),
            };
        }
    }

    let mut total_distance: u32 = 0;
    let mut similarity_score = 0;
    for (l, r) in left_sorted.into_iter().zip(right_sorted) {
        total_distance += l.abs_diff(r);

        if let Some(count) = right_counts.get(&l) {
            similarity_score += l * count;
        }
    }

    // Part 1 result
    println!("Total distance: {}", total_distance);

    // Part 2 result
    println!("Similarity Score {}", similarity_score);
}
