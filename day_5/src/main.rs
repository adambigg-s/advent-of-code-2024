


use std::{env, fs, process, time::Instant};



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

    let solution: Solution = Solution::construct();

    let s1 = Instant::now();
    let part_one = solution.solve_one(&buffer);
    let p1 = s1.elapsed();

    let s2 = Instant::now();
    let part_two = solution.solve_two(&buffer);
    let p2 = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__");
    println!("part one: {}\ntime one: {:#?}", part_one, p1);
    println!();
    println!("part two: {}\ntime two: {:#?}", part_two, p2);
    println!();
}

struct Solution {}

impl Solution {
    fn construct() -> Solution {
        Solution {}
    }

    fn solve_one(&self, buffer: &str) -> i32 {
        let mut keys: Vec<(i32, i32)> = Vec::new();
        let mut pages: Vec<Vec<i32>> = Vec::new();
        buffer.lines().for_each(|line| {
            if line.contains('|') {
                let mut tokens = line.split('|');
                let insertion: (i32, i32) = (
                    tokens.next().unwrap().parse::<i32>().unwrap(),
                    tokens.next().unwrap().parse::<i32>().unwrap()
                );
                keys.push(insertion);
            } else if !line.is_empty() {
                let tokens: Vec<i32> = line
                    .split(',')
                    .map(|tok| tok.parse::<i32>().unwrap())
                    .collect();
                pages.push(tokens);
            }
        });

        let mut total: i32 = 0;
        for page in &pages {
            let mut valid: bool = true;

            for (first, last) in &keys {
                if page.contains(first) && page.contains(last) {
                    let first_pos = page.iter().position(|x| x == first).unwrap();
                    let last_pos = page.iter().position(|x| x == last).unwrap();

                    if first_pos > last_pos {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                total += page[page.len() / 2];
            }
        }

        total
    }

    fn solve_two(&self, buffer: &str) -> i32 {
        let mut keys: Vec<(i32, i32)> = Vec::new();
        let mut pages: Vec<Vec<i32>> = Vec::new();
        buffer.lines().for_each(|line| {
            if line.contains('|') {
                let mut tokens = line.split('|');
                let insertion: (i32, i32) = (
                    tokens.next().unwrap().parse::<i32>().unwrap(),
                    tokens.next().unwrap().parse::<i32>().unwrap()
                );
                keys.push(insertion);
            } else if !line.is_empty() {
                let tokens: Vec<i32> = line
                    .split(',')
                    .map(|tok| tok.parse::<i32>().unwrap())
                    .collect();
                pages.push(tokens);
            }
        });

        let mut total: i32 = 0;
        for page in &pages {
            let mut valid: bool = true;

            for (first, last) in &keys {
                if page.contains(first) && page.contains(last) {
                    let first_pos = page.iter().position(|x| x == first).unwrap();
                    let last_pos = page.iter().position(|x| x == last).unwrap();

                    if first_pos > last_pos {
                        valid = false;
                        break;
                    }
                }
            }

            if !valid {
                let mut reorder: Vec<i32> = page.clone();
                self.reorder(&mut reorder, &keys);
                total += reorder[reorder.len() / 2];
            }
        }

        total
    }

    fn reorder(&self, list: &mut [i32], key: &[(i32, i32)]) {
        let mut changed: bool = true;

        while changed {
            changed = false;
            for &(first, last) in key {
                if let (Some(first_pos), Some(last_pos)) = (
                    list.iter().position(|&x| x == first),
                    list.iter().position(|&x| x == last),
                ) 
                {
                    if first_pos > last_pos {
                        list.swap(first_pos, last_pos);
                        changed = true;
                    }
                }
            }
        }
    }
}
