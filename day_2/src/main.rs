


use std::{env, fs, process};



fn main() {
    let envs: Vec<String> = env::args().collect();
    let mut testing: bool = false;
    let file_path: &str = envs.get(1).map(|path| path.as_str()).unwrap_or_else(|| {
        testing = true;
        "testing.txt"
    });
    let buffer: String = fs::read_to_string(file_path).unwrap_or_else(|err| {
        println!("buffer read error: {err}");
        process::exit(3);
    });
    if testing { println!("{buffer}"); }

    let part_one = Solution::solve_one(&buffer);
    let part_two = Solution::solve_two(&buffer);
    println!(
        "
            part one: {part_one}\n
            part two: {part_two}\n
        "
    );
}

struct Solution;

impl Solution {
    fn solve_one(buffer: &str) -> i32 {
        let mut safe: i32 = 0;
        for report in buffer.lines() {
            let data: Vec<i32> = report.split_whitespace().map(|d| d.parse::<i32>().unwrap()).collect();
            if Self::is_safe(&data) {
                safe += 1;
            }
        }

        safe
    }

    fn solve_two(buffer: &str) -> i32 {
        let mut safe: i32 = 0;
        for report in buffer.lines() {
            let data: Vec<i32> = report.split_whitespace().map(|d| d.parse::<i32>().unwrap()).collect();
            if Self::is_safe(&data) {
                safe += 1;
            } else {
                for i in 0..data.len() {
                    let mut cloned = data.clone();
                    cloned.remove(i);
                    if Self::is_safe(&cloned) {
                        safe += 1;
                        break;
                    }
                }
            }
        }

        safe
    }

    fn is_safe(report: &[i32]) -> bool {
        let differences: Vec<i32> = report.windows(2).map(|two| two[1] - two[0]).collect();
        let increasing = differences.iter().all(|&diff| {
            diff > 0 && diff < 4
        });
        let decreasing = differences.iter().all(|&diff| {
            diff < 0 && diff > -4
        });

        increasing || decreasing
    }
}

