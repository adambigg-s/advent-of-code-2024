


use std::{collections::HashMap, env, fmt::Display, fs, process};



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

    let part_1 = Solution::solve_one(&buffer);
    println!("part 1: {part_1}");
    let part_2 = Solution::solve_two(&buffer);
    println!("part 2: {part_2}");
}

struct Solution;

impl Solution {
    fn solve_one(buffer: &str) -> i32 {
        let mut list1: Vec<i32> = Vec::new();
        let mut list2: Vec<i32> = Vec::new();
        let mut diff: i32 = 0;

        for line in buffer.lines() {
            let mut nums = line.split_whitespace();
            let first = nums.nth(0).unwrap();
            let last = nums.nth(0).unwrap();
            list1.push(first.parse::<i32>().unwrap());
            list2.push(last.parse::<i32>().unwrap());
        }

        list1.sort_unstable();
        list2.sort_unstable();

        {
            debug_assert!(list1.len() == list2.len());
        }
        for idx in 0..list1.len() {
            diff += (list1[idx] - list2[idx]).abs();
        }

        diff
    }

    fn solve_two(buffer: &str) -> i32 {
        let mut list: Vec<i32> = Vec::new();
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        let mut diff: i32 = 0;

        for line in buffer.lines() {
            let mut nums = line.split_whitespace();
            let first = nums.nth(0).unwrap();
            let last = nums.nth(0).unwrap();
            list.push(first.parse::<i32>().unwrap());
            *hashmap.entry(last.parse::<i32>().unwrap()).or_insert(0) += 1;
        }

        for num in &list {
            let freq = hashmap.get(num).unwrap_or(&0);
            diff += freq * num;
        }

        diff
    }
}
