


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
    if testing {
        println!("{buffer}");
    }

    let solution: Solution = Solution::construct(&buffer);

    let instances_one: i32 = solution.solve_one();
    println!("part one: {instances_one}");
    let instances_two: i32 = solution.solve_two();
    println!("part two: {instances_two}");
}

struct Solution {
    grid: Vec<Vec<char>>,
    m: usize, 
    n: usize,
}

impl Solution {
    fn construct(buffer: &str) -> Solution {
        let grid = Self::gridify(buffer);
        let m = grid.len();
        let n = grid.first().unwrap().len();

        Solution { grid, m, n }
    }

    fn solve_one(&self) -> i32 {
        let mut occurances: i32 = 0;
        for i in 0..self.m {
            for j in 0..self.n {
                occurances += self.search(i, j);
            }
        }

        occurances
    }

    fn solve_two(&self) -> i32 {
        let mut occurances: i32 = 0;
        for i in 0..self.m {
            for j in 0..self.n {
                let found = self.cross_search(i, j);
                occurances += found / 2;
            }
        }

        occurances
    }

    fn cross_search(&self, i: usize, j: usize) -> i32 {
        let mut occurances: i32 = 0;
        let directions: [(isize, isize); 4]= [
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        let target: [char; 3] = ['M', 'A', 'S'];

        if self.grid[i][j] != target[1] {
            return 0;
        }

        for (dx, dy) in directions {
            let (nx, ny) = Self::idx(i, j, dx, dy);
            let (nxp, nyp) = Self::idx(i, j, -dx, -dy);
            if self.inbounds(nx, ny) 
                && self.grid[ny][nx] == target[0] 
                && self.inbounds(nxp, nyp) 
                && self.grid[nyp][nxp] == target[2] 
            {
                occurances += 1;
            }
        }
        
        occurances
    }

    fn search(&self, i: usize, j: usize) -> i32 {
        let mut occurances: i32 = 0;
        let directions: [(isize, isize); 8] = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        let target: [char; 4] = ['X', 'M', 'A', 'S'];

        for (dx, dy) in directions {
            if self.grid[i][j] == target[0] {
                let (nx, ny) = Self::idx(i, j, dx, dy);
                if self.inbounds(nx, ny) && self.grid[ny][nx] == target[1] {
                    let (nx, ny) = Self::idx(ny, nx, dx, dy);
                    if self.inbounds(nx, ny) && self.grid[ny][nx] == target[2] {
                        let (nx, ny) = Self::idx(ny, nx, dx, dy);
                        if self.inbounds(nx, ny) && self.grid[ny][nx] == target[3] {
                            occurances += 1;
                        }
                    }
                }
            }
        }

        occurances
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.n && y < self.m
    }

    fn idx(y: usize, x: usize, dx: isize, dy: isize) -> (usize, usize) {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        (nx, ny)
    }

    fn gridify(buffer: &str) -> Vec<Vec<char>> {
        buffer
            .lines()
            .map(|line| {
                line
                    .chars()
                    .collect()
            })
            .collect()
    }
}
