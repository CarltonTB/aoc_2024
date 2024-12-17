use std::collections::HashSet;
use std::fs;
use std::process;

pub fn solve() {
    let input_path = format!("{}/src/day4/input.txt", env!("CARGO_MANIFEST_DIR"));
    let contents = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Problem parsing input file: {err}");
        process::exit(1);
    });

    // let test_input = "\
    //     MMMSXXMASM\n \
    //     MSAMXMSMSA\n \
    //     AMXSXMAAMM\n \
    //     MSAMASMSMX\n \
    //     XMASAMXAMM\n \
    //     XXAMMXXAMA\n \
    //     SMSMSASXSS\n \
    //     SAXAMASAAA\n \
    //     MAMMMXMMMM\n \
    //     MXMXAXMASX\n";
    //
    // contents = test_input.to_string();

    // construct a 2D vec of the input
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in contents.split_whitespace() {
        if !line.is_empty() {
            matrix.push(line.chars().collect());
        };
    }

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
}
