


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
    if testing {
        println!("{buffer}");
    }

    let mut solution: Solution = Solution::construct(&buffer);

    let s1 = Instant::now();
    let part_one = solution.solve_one(testing);
    let p1 = s1.elapsed();

    let s2 = Instant::now();
    let part_two = solution.solve_two();
    let p2 = s2.elapsed();

    println!("part one: {}\ntime one: {:#?}", &part_one, &p1);
    println!();
    println!("part two: {}\ntime two: {:#?}", &part_two, &p2);
    println!();
}

#[derive(PartialEq, Clone, Copy)]
struct Direction {
    dx: isize,
    dy: isize,
}

impl Direction {
    fn default() -> Direction {
        Direction { dx: 0, dy: -1 }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Barrier,
    Guard(Direction),
}

impl Cell {
    fn from_chr(chr: char) -> Cell {
        match chr {
            '.' => Self::Empty,
            '#' => Self::Barrier,
            '^' => Self::Guard(Direction::default()),
            _ => {
                panic!("invalid character in grid");
            }
        }
    }

    fn is_empty(&self) -> bool {
        *self == Self::Empty
    }

    fn is_guard(&self) -> bool {
        matches!(*self, Self::Guard(_))
    }
}

struct Solution {
    grid: Vec<Vec<Cell>>,
    y: usize,
    x: usize,
}

impl Solution {
    fn construct(buffer: &str) -> Solution {
        let grid = Self::gridify(buffer);
        let m = grid.len();
        let n = grid.first().unwrap().len();
        Solution { grid, y: m, x: n }
    }

    fn solve_one(&mut self, testing: bool) -> i32 {
        let mut seen: Vec<Vec<bool>> = vec![vec![false; self.x]; self.y];
        let mut grid: Vec<Vec<Cell>> = self.grid.clone();
        let (mut gx, mut gy) = self.find_guard();
        let (mut dx, mut dy): (isize, isize) = (0, -1);

        loop {
            seen[gy][gx] = true;
            if let Some((tx, ty)) = self.newdex(gx, gy, dx, dy) {
                if grid[ty][tx].is_empty() {
                    grid[ty][tx] = grid[gy][gx];
                    grid[gy][gx] = Cell::Empty;
                    (gx, gy) = (tx, ty);
                } else {
                    self.rotate(&mut dx, &mut dy);
                }
            } else {
                break;
            }
        }

        if testing {
            for vec in &seen {
                for ele in vec {
                    if *ele {
                        print!("X");
                    } else {
                        print!(".")
                    }
                }
                println!();
            }
            println!();
            println!();
            for vec in &self.grid {
                for ele in vec {
                    if ele.is_empty() {
                        print!(".");
                    } else {
                        print!("o");
                    }
                }
                println!();
            }
        }

        seen.iter().flatten().filter(|ele| **ele).count() as i32
    }

    fn solve_two(&self) -> i32 {
        let mut seen: Vec<Vec<bool>> = vec![vec![false; self.x]; self.y];
        let mut grid: Vec<Vec<Cell>> = self.grid.clone();
        let (mut gx, mut gy) = self.find_guard();
        let (mut dx, mut dy): (isize, isize) = (0, -1);

        let mut blockades: i32 = 0;

        loop {
            seen[gy][gx] = true;
            if let Some((tx, ty)) = self.newdex(gx, gy, dx, dy) {
                if grid[ty][tx].is_empty() {
                    grid[ty][tx] = grid[gy][gx];
                    grid[gy][gx] = Cell::Empty;
                    (gx, gy) = (tx, ty);
                } else {
                    self.rotate(&mut dx, &mut dy);
                }
            } else {
                break;
            }
        }

        blockades
    }

    fn rotate(&self, dx: &mut isize, dy: &mut isize) {
        match (*dx, *dy) {
            (0, -1) => {
                *dx = 1;
                *dy = 0
            }
            (1, 0) => {
                *dx = 0;
                *dy = 1
            }
            (0, 1) => {
                *dx = -1;
                *dy = 0
            }
            (-1, 0) => {
                *dx = 0;
                *dy = -1
            }
            _ => panic!("should always be ones and zeros"),
        };
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.x && y < self.y
    }

    fn newdex(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        if self.inbounds(nx, ny) {
            Some((nx, ny))
        } else {
            None
        }
    }

    fn find_guard(&self) -> (usize, usize) {
        for i in 0..self.y {
            for j in 0..self.x {
                if self.grid[i][j].is_guard() {
                    return (j, i);
                }
            }
        }
        panic!("shouldn't reach here");
    }

    fn gridify(buffer: &str) -> Vec<Vec<Cell>> {
        buffer
            .lines()
            .map(|line| line.chars().map(|chr| Cell::from_chr(chr)).collect())
            .collect()
    }
}
