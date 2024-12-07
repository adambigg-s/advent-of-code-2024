


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
    let part_one = solution.solve_one();
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
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn velocity_default() -> Vec2 {
        Vec2 { x: 0, y: -1 }
    }

    fn from(x: isize, y: isize) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Barrier,
    Guard(Vec2),
}

impl Cell {
    fn from_chr(chr: char) -> Cell {
        match chr {
            '.' => Self::Empty,
            '#' => Self::Barrier,
            '^' => Self::Guard(Vec2::velocity_default()),
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

    fn solve_one(&mut self) -> i32 {
        let mut seen: Vec<(usize, usize)> = Vec::new();
        self.populate_seen(&mut seen);

        seen.len() as i32
    }

    fn solve_two(&self) -> i32 {
        let mut seen: Vec<(usize, usize)> = Vec::new();
        self.populate_seen(&mut seen);

        let mut blockade: i32 = 0;
        for (seen_x, seen_y) in &seen {
            let mut grid = self.grid.clone();
            let mut path: Vec<Vec<Option<Vec2>>> = vec![vec![None; self.x]; self.y];
            let (mut gx, mut gy) = self.find_guard();
            let (mut vx, mut vy) = (Vec2::velocity_default().x, Vec2::velocity_default().y);
            grid[*seen_y][*seen_x] = Cell::Barrier;
            let mut moved: bool = false;
            loop {
                if let Some(vec) = path[gy][gx] {
                    if vec == Vec2::from(vx, vy) && moved {
                        blockade += 1;
                        break;
                    }
                } else {
                    path[gy][gx] = Some(Vec2::from(vx, vy));
                }
                if let Some((tx, ty)) = self.newdex(gx, gy, vx, vy) {
                    if grid[ty][tx].is_empty() {
                        grid[ty][tx] = grid[gy][gx];
                        grid[gy][gx] = Cell::Empty;
                        (gx, gy) = (tx, ty);
                        moved = true;
                    } else {
                        self.rotate(&mut vx, &mut vy);
                        moved = false;
                    }
                } else {
                    break;
                }
            }
        }
        
        blockade
    }

    fn populate_seen(&self, seen: &mut Vec<(usize, usize)>) {
        let mut grid: Vec<Vec<Cell>> = self.grid.clone();
        let (mut gx, mut gy) = self.find_guard();
        let (mut vx, mut vy) = (Vec2::velocity_default().x, Vec2::velocity_default().y);
        loop {
            if !seen.contains(&(gx, gy)) {
                seen.push((gx, gy));
            }
            if let Some((tx, ty)) = self.newdex(gx, gy, vx, vy) {
                if grid[ty][tx].is_empty() {
                    grid[ty][tx] = grid[gy][gx];
                    grid[gy][gx] = Cell::Empty;
                    (gx, gy) = (tx, ty);
                } else {
                    self.rotate(&mut vx, &mut vy);
                }
            } else {
                break;
            }
        }
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
            .map(|line| line.chars().map(Cell::from_chr).collect())
            .collect()
    }
}
