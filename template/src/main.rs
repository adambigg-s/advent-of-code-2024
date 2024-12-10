


#[allow(unused_imports)]
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Div, Sub},
    env, fs, process,
    time::Instant,
};



type Num = i32;

fn main()
{
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

    let solution: Solution = Solution::construct(&buffer);

    let s1 = Instant::now();
    let part_one = solution.solve_one();
    let p1 = s1.elapsed();

    let s2 = Instant::now();
    let part_two = solution.solve_two();
    let p2 = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}", part_one, p1);
    println!();
    println!("part two: {}\ntime two: {:#?}", part_two, p2);
    println!();
}

struct Solution
{
    
}

impl Solution
{
    fn construct(buffer: &str) -> Solution
    {
        Solution {}
    }

    fn solve_one(&self) -> Num
    {

        0
    }

    fn solve_two(&self) -> Num
    {

        0
    }
}
