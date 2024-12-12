


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

struct GeoData
{
    area: Num,
    perimeter: Num,
}

impl GeoData
{
    fn new() -> GeoData
    {
        GeoData { area: 0, perimeter: 0 }
    }
}

struct Solution
{
    grid: Vec<Vec<char>>,
    xdim: usize,
    ydim: usize,
}

impl Solution
{
    fn construct(buffer: &str) -> Solution
    {
        let grid: Vec<Vec<char>> = buffer.lines().map(|line| {
            line.chars().collect()
        }).collect();
        let ydim = grid.len();
        let xdim = grid.first().unwrap().len();
        Solution { grid, xdim, ydim }
    }

    fn solve_one(&self) -> Num
    {
        let mut seen: Vec<Vec<bool>> = vec![vec![false; self.xdim]; self.ydim];
        let mut plots: Vec<GeoData> = Vec::new();

        (0..self.ydim).for_each(|y| {
            (0..self.xdim).for_each(|x| {
                if !seen[y][x] {
                    let tag = self.grid[y][x];
                    let mut data: GeoData = GeoData::new();
                    self.recursive_search(x, y, tag, &mut seen, &mut data);
                    plots.push(data);
                }
            })
        });

        let mut total: Num = 0;
        for plot in plots {
            total += plot.area * plot.perimeter;
        }

        total
    }

    fn recursive_search(&self, x: usize, y: usize, tag: char, seen: &mut Vec<Vec<bool>>, data: &mut GeoData)
    {
        if self.grid[y][x] != tag || seen[y][x] {
            return;
        }

        seen[y][x] = true;
        data.area += 1;
        for (dx, dy) in get_directions() {
            if let Some((nx, ny)) = self.idx(x, y, dx, dy) {
                if self.grid[ny][nx] != tag {
                    data.perimeter += 1;
                }
                self.recursive_search(nx, ny, tag, seen, data);
            } else {
                data.perimeter += 1;
            }
        }
    }

    fn idx(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)>
    {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;
        if nx < self.xdim && ny < self.ydim {
            Some((nx, ny))
        } else {
            None
        }
    }

    fn solve_two(&self) -> Num
    {

        0
    }
}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, 1), (0, -1),
    (1, 0), (-1, 0),
];

fn get_directions() -> [(isize, isize); 4]
{
    DIRECTIONS
}
