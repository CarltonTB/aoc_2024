use std::fs;
use std::process;

const MIN_DIFF: u32 = 1;
const MAX_DIFF: u32 = 3;

struct ReportAnalysis {
    valid: bool,
    safe: bool,
}

fn analyze_report(report_line: &str) -> ReportAnalysis {
    let mut levels = report_line.split_whitespace();
    let mut increasing = true;

    let mut prev_level = levels.next();
    let mut cur_level = levels.next();

    let mut valid = true;
    match (prev_level, cur_level) {
        (None, None) => {
            println!("Invalid line");
            valid = false;
        }
        (Some(i), Some(j)) => {
            let prev: i32 = i.parse().expect("Not a valid number");
            let next: i32 = j.parse().expect("Not a valid number");
            increasing = next - prev > 0;
        }
        _ => {
            valid = false;
        }
    }

    if !valid {
        return ReportAnalysis { valid, safe: false };
    }

    let mut safe = true;
    while cur_level != None {
        let prev_level_val: i32 = prev_level.unwrap().parse().expect("Not a valid number");
        let cur_level_val: i32 = cur_level.unwrap().parse().expect("Not a valid number");

        let diff: i32 = cur_level_val - prev_level_val;
        let abs_diff: u32 = cur_level_val.abs_diff(prev_level_val);

        if (abs_diff < MIN_DIFF || abs_diff > MAX_DIFF)
            || (diff < 0 && increasing)
            || (diff > 0 && !increasing)
        {
            safe = false;
            break;
        }

        prev_level = cur_level;
        cur_level = levels.next();
    }

    ReportAnalysis { valid: true, safe }
}

pub fn solve() {
    let input_path = format!("{}/src/day2/input.txt", env!("CARGO_MANIFEST_DIR"));
    let contents = fs::read_to_string(input_path).unwrap_or_else(|err| {
        eprintln!("Problem parsing input file: {err}");
        process::exit(1);
    });

    let mut safe_count = 0;
    let mut safe_with_dampener_count = 0;

    for line in contents.split('\n') {
        let analysis: ReportAnalysis = analyze_report(line);

        if !analysis.valid {
            continue;
        }

        if analysis.safe {
            safe_count += 1;
        } else {
            let report: Vec<&str> = line.split_whitespace().collect();
            for i in 0..report.len() {
                let mut report_to_check: Vec<&str> = report.clone();
                report_to_check.remove(i);

                let analysis: ReportAnalysis = analyze_report(&report_to_check.join(" "));
                if analysis.valid && analysis.safe {
                    safe_with_dampener_count += 1;
                    break;
                }
            }
        }
    }

    println!("Safe reports: {}", safe_count);
    println!(
        "Safe reports with Problem Dampener: {}",
        safe_with_dampener_count
    );
    println!(
        "Total safe reports: {}",
        safe_count + safe_with_dampener_count
    );
}
