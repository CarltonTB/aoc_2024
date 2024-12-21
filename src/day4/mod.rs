use std::collections::HashSet;
use std::fs;
use std::process;

pub fn solve() {
    let input_path = format!("{}/src/day4/input.txt", env!("CARGO_MANIFEST_DIR"));
    let contents = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Problem parsing input file: {err}");
        process::exit(1);
    });

    // construct a 2D vec of the input
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in contents.split_whitespace() {
        if !line.is_empty() {
            matrix.push(line.chars().collect());
        };
    }

    // Part 1
    // Use a 4 x 4 sliding window, store the start and end coordinates of
    // every XMAS found, then count the total unique XMAS's at the end
    let xmas: String = String::from("XMAS");
    let xmas_rev: String = xmas.chars().rev().collect();
    let mut found: HashSet<Vec<Vec<usize>>> = HashSet::new();

    for i in 0..matrix.len() - 3 {
        for j in 0..matrix[0].len() - 3 {
            // Check horizontal
            for k in 0..4 {
                let start = vec![i + k, j];
                let end = vec![i + k, j + 3];
                let mut coords = vec![start, end];
                let word: String = matrix[i + k][j..j + 4].iter().collect();
                if word == xmas {
                    found.insert(coords);
                } else if word == xmas_rev {
                    coords.reverse();
                    found.insert(coords);
                }
            }

            // Check vertical
            for k in 0..4 {
                let start = vec![i, j + k];
                let end = vec![i + 3, j + k];
                let mut coords: Vec<Vec<usize>> = vec![start, end];
                let mut word: String = String::new();
                for l in 0..4 {
                    word.push(matrix[i + l][j + k]);
                }
                if word == xmas {
                    found.insert(coords);
                } else if word == xmas_rev {
                    coords.reverse();
                    found.insert(coords);
                }
            }

            // Check diagonal top left to bottom right
            let start = vec![i, j];
            let end = vec![i + 3, j + 3];
            let mut coords = vec![start, end];
            let mut word: String = String::new();
            for k in 0..4 {
                word.push(matrix[i + k][j + k]);
            }
            if word == xmas {
                found.insert(coords);
            } else if word == xmas_rev {
                coords.reverse();
                found.insert(coords);
            }

            // Check diagonal top right to bottom left
            let start = vec![i + 3, j];
            let end = vec![i, j + 3];
            let mut coords = vec![start, end];
            let mut word: String = String::new();
            for k in 0..4 {
                word.push(matrix[i + k][j + (3 - k)]);
            }
            if word == xmas {
                found.insert(coords);
            } else if word == xmas_rev {
                coords.reverse();
                found.insert(coords);
            }
        }
    }
    println!("XMAS count: {}", found.len());

    // Part 2
    // Use a 3 x 3 sliding window, count each MAS | SAM pair in the form of an X
    let mas: String = String::from("MAS");
    let mas_rev: String = mas.chars().rev().collect();
    let mut mas_count = 0;

    for i in 0..matrix.len() - 2 {
        for j in 0..matrix[0].len() - 2 {
            // Check diagonal top left to bottom right
            let mut word1: String = String::new();
            for k in 0..3 {
                word1.push(matrix[i + k][j + k]);
            }

            // Check diagonal top right to bottom left
            let mut word2: String = String::new();
            for k in 0..3 {
                word2.push(matrix[i + k][j + (2 - k)]);
            }

            if (word1 == mas || word1 == mas_rev) && (word2 == mas || word2 == mas_rev) {
                mas_count += 1;
            }
        }
    }

    println!("X-MAS count: {}", mas_count);
}
