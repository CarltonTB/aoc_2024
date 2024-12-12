use regex::Regex;
use std::fs;
use std::process;

pub fn solve() {
    let input_path = format!("{}/src/day3/input.txt", env!("CARGO_MANIFEST_DIR"));
    let contents = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Problem parsing input file: {err}");
        process::exit(1);
    });

    let mut mul_sum: u32 = 0;
    let mut mul_sum_with_conditionals: u32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    for capture in re.captures_iter(&contents) {
        if capture.get(0).unwrap().as_str() == "do()" {
            enabled = true;
            continue;
        } else if capture.get(0).unwrap().as_str() == "don't()" {
            enabled = false;
            continue;
        }

        let n: u32 = capture
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .expect("first capture group to be a valid number");
        let m: u32 = capture
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .expect("second capture group to be a valid number");

        let product = n * m;
        if enabled {
            mul_sum_with_conditionals += product;
        }
        mul_sum += product;
    }

    println!("Sum of multiplications: {}", mul_sum);
    println!(
        "Sum of multiplications accounting for conditionals: {}",
        mul_sum_with_conditionals
    );
}
