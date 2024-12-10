use std::fs;
use std::process;

pub fn solve() {
    let input_path = format!("{}/src/day2/input.txt", env!("CARGO_MANIFEST_DIR"));
    let contents = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Problem parsing input file: {err}");
        process::exit(1);
    });

    let min_diff = 1;
    let max_diff = 3;
    let mut safe_count = 0;

    for (i, line) in contents.split('\n').enumerate() {
        let mut levels = line.split_whitespace();

        let mut increasing = true;
        let mut safe = true;

        let mut prev_level = levels.next();
        let mut cur_level = levels.next();

        match (prev_level, cur_level) {
            (None, None) => {
                println!("{} is an ivalid line", i);
                continue; // invalid line, so ignore it
            }
            (Some(i), Some(j)) => {
                let prev: i32 = i.parse().expect("Not a valid number");
                let next: i32 = j.parse().expect("Not a valid number");
                increasing = next - prev > 0;
            }
            _ => {}
        }

        while cur_level != None {
            let prev_level_val: i32 = prev_level.unwrap().parse().expect("Not a valid number");
            let cur_level_val: i32 = cur_level.unwrap().parse().expect("Not a valid number");

            let diff: i32 = cur_level_val - prev_level_val;
            let abs_diff: u32 = cur_level_val.abs_diff(prev_level_val);

            if abs_diff < min_diff || abs_diff > max_diff {
                safe = false;
                break;
            } else if diff < 0 && increasing {
                safe = false;
                break;
            } else if diff > 0 && !increasing {
                safe = false;
                break;
            }

            prev_level = cur_level;
            cur_level = levels.next();
        }

        if safe {
            safe_count += 1;
        }
    }

    println!("Safe reports: {}", safe_count)
}
