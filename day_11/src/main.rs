


use std::default;
#[allow(unused_imports)]
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Div, Sub},
    env, fs, process,
    time::Instant,
};



type Num = i64;

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

    let s1: Instant = Instant::now();
    let mut solution: Solution = Solution::construct(&buffer);
    let part_one: Num = solution.solve_one();
    let p1: std::time::Duration = s1.elapsed();

    let s2: Instant = Instant::now();
    let mut solution: Solution = Solution::construct(&buffer);
    let part_two: Num = solution.solve_two();
    let p2: std::time::Duration = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}", part_one, p1);
    println!();
    println!("part two: {}\ntime two: {:#?}", part_two, p2);
    println!();
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Stone
{
    tag: Num,
}

impl Stone
{
    fn cons(tag: Num) -> Stone
    {
        Stone { tag }
    }
}

struct Solution
{
    stones: Vec<Stone>,
}

impl Solution
{
    fn construct(buffer: &str) -> Solution
    {
        let stones: Vec<Stone> = buffer.split_whitespace().map(|num| {
            let tag = num.parse::<Num>().unwrap();
            Stone::cons(tag)
        }).collect();

        Solution { stones }
    }

    fn solve_one(&mut self) -> Num
    {
        (0..25).for_each(|_| {
            self.blink();
        });

        self.stones.len() as Num
    }

    fn solve_two(&mut self) -> Num
    {
        let mut stones: HashMap<Num, Num> = HashMap::new();

        for stone in &self.stones {
            *stones.entry(stone.tag).or_default() += 1;
        }

        for _ in 0..75 {
            let mut changes: HashMap<Num, Num> = HashMap::new();

            for (&key, &count) in &stones {
                if count <= 0 {
                    continue;
                }

                if key == 0 {
                    *changes.entry(1).or_default() += count;
                }
                else if key.to_string().len() % 2 == 0 {
                    let str = key.to_string();
                    let (first, second) = str.split_at(str.len() / 2);
                    *changes.entry(first.parse::<Num>().unwrap()).or_default() += count;
                    *changes.entry(second.parse::<Num>().unwrap()).or_default() += count;
                }
                else {
                    *changes.entry(key * 2024).or_default() += count;
                }

                *changes.entry(key).or_default() -= count;
            }

            for (key, change) in changes {
                *stones.entry(key).or_default() += change;

                if stones[&key] <= 0 {
                    stones.remove(&key);
                }
            }
        }

        stones.values().filter(|&&val| val > 0).sum()
    }

    fn blink(&mut self)
    {
        for i in 0..self.stones.len() {
            let stone: &mut Stone = &mut self.stones[i];
            if stone.tag == 0 {
                stone.tag = 1;
            }
            else if stone.tag.to_string().len() % 2 == 0 {
                let tag_str = stone.tag.to_string();
                let (first, second) = tag_str.split_at(tag_str.len() / 2);
                stone.tag = first.parse::<Num>().unwrap();
                self.stones.push(Stone::cons(second.parse::<Num>().unwrap()));
            }
            else {
                stone.tag *= 2024;
            }
        }
    }
}
