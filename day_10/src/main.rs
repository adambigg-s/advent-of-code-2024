


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

    let s1: Instant = Instant::now();
    let part_one: Num = solution.solve_one();
    let p1: std::time::Duration = s1.elapsed();

    let s2: Instant = Instant::now();
    let part_two: Num = solution.solve_two();
    let p2: std::time::Duration = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}", part_one, p1);
    println!();
    println!("part two: {}\ntime two: {:#?}", part_two, p2);
    println!();
}

struct Solution
{
    grid: Vec<Vec<Num>>,
    xdim: usize,
    ydim: usize,
}

impl Solution
{
    fn construct(buffer: &str) -> Solution
    {
        let grid: Vec<Vec<Num>> = buffer.lines().map(|line| {
            line.chars().map(|chr| {
                chr.to_digit(10).unwrap_or(333) as Num
            }).collect()
        }).collect();
        let ydim: usize = grid.len();
        let xdim: usize = grid.first().unwrap().len();

        Solution { grid, xdim, ydim }
    }

    fn solve_one(&self) -> Num
    {
        let trailheads: Vec<(usize, usize)> = self.populate_trailheads();

        let mut scores: Num = 0;
        for trailhead in trailheads {
            let (x, y): (usize, usize) = trailhead;
            let curr: i32 = 0;
            let target: i32 = 9;
            let mut seen: Vec<Vec<bool>> = vec![vec![false; self.xdim]; self.ydim];
            self.recursive_search_trailhead(x, y, &mut scores, curr, target, &mut seen);
        }

        scores
    }

    fn solve_two(&self) -> Num
    {
        let trailheads: Vec<(usize, usize)> = self.populate_trailheads();

        let mut ratings: Num = 0;
        for trailhead in trailheads {
            let (x, y): (usize, usize) = trailhead;
            let curr: i32 = 0;
            let target: i32 = 9;
            self.recursive_search_trailhead_path(x, y, &mut ratings, curr, target);
        }

        ratings
    }

    fn recursive_search_trailhead(&self, x: usize, y: usize, found: &mut Num, curr: Num, target: Num, seen: &mut Vec<Vec<bool>>)
    {
        if self.grid[y][x] == target && !seen[y][x] {
            seen[y][x] = true;
            *found += 1;
            return;
        }

        for (dx, dy) in get_directions() {
            if let Some((nx, ny)) = self.idx(x, y, dx, dy) {
                if self.grid[ny][nx] == curr + 1 {
                    self.recursive_search_trailhead(nx, ny, found, curr + 1, target, seen);
                }
            }
        }
    }

    fn recursive_search_trailhead_path(&self, x: usize, y: usize, found: &mut Num, curr: Num, target: Num)
    {
        if self.grid[y][x] == target {
            *found += 1;
            return;
        }

        for (dx, dy) in get_directions() {
            if let Some((nx, ny)) = self.idx(x, y, dx, dy) {
                if self.grid[ny][nx] == curr + 1 {
                    self.recursive_search_trailhead_path(nx, ny, found, curr + 1, target);
                }
            }
        }
    }

    fn idx(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)>
    {
        let nx: usize = (x as isize + dx) as usize;
        let ny: usize = (y as isize + dy) as usize;
        if nx < self.xdim && ny < self.ydim {
            Some((nx, ny))
        } else {
            None
        }
    }

    fn populate_trailheads(&self) -> Vec<(usize, usize)>
    {
        let mut trailheads: Vec<(usize, usize)> = Vec::new();
        (0..self.ydim).for_each(|y| {
            (0..self.xdim).for_each(|x| {
                if self.grid[y][x] == 0 {
                    trailheads.push((x, y));
                }
            })
        });

        trailheads
    }
}

const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0), (-1, 0),
    (0, 1), (0, -1),
];

fn get_directions() -> [(isize, isize); 4]
{
    DIRECTIONS
}
