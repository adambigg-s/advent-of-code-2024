


use std::{collections::{BinaryHeap, HashSet}, env, fs, time::Instant};

use day_16::*;



fn main()
{
    let envs: Vec<String> = env::args().collect();
    let mut testing: bool = false;
    let file_path: &str = envs.get(1).map(|path| path.as_str()).unwrap_or_else(|| {
        testing = true;
        "testing.txt"
    });
    let buffer: String = fs::read_to_string(file_path).unwrap();
    if testing { println!("{buffer}"); }

    let s1: Instant = Instant::now();
    let solution: Solution = Solution::construct(&buffer);
    let part_one: Int = solution.solve_one();
    let p1: std::time::Duration = s1.elapsed();

    let s2: Instant = Instant::now();
    let solution: Solution = Solution::construct(&buffer);
    let part_two: Int = solution.solve_two();
    let p2: std::time::Duration = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}\n", part_one, p1);
    println!("part two: {}\ntime two: {:#?}\n", part_two, p2);
}

struct Solution
{
    maze: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Solution
{
    fn construct(buffer: &str) -> Solution
    {
        let maze: Vec<Vec<char>> = buffer.trim().lines().map(|line| {
            line.chars().collect()
        }).collect();
        let width = maze.first().unwrap().len();
        let height = maze.len();

        Solution { maze, width, height }
    }

    fn solve_one(&self) -> Int
    {
        let start: Vec2<usize> = self.find_tile(start()).unwrap();
        let end: Vec2<usize> = self.find_tile(end()).unwrap();

        self.bread_fish(start, end)
    }

    fn bread_fish(&self, start: Vec2<usize>, end: Vec2<usize>) -> Int
    {
        let mut queue: BinaryHeap<(Int, State)> = BinaryHeap::new();
        let mut visited: HashSet<(Vec2<usize>, Vec2<isize>)> = HashSet::new();

        for direc in get_directions() {
            if let Some(new_pos) = self.idx(&start, &direc) {
                if !self.maze[new_pos.y][new_pos.x].is_wall() {
                    queue.push((0, State::cons(new_pos, direc, 0)));
                    visited.insert((start, direc));
                }
            }
        }

        while let Some((current_score, state)) = queue.pop() {
            if state.pos == end {
                return state.score;
            }

            if let Some(new_pos) = self.idx(&state.pos, &state.vel) {
                if !self.maze[new_pos.y][new_pos.x].is_wall() && visited.insert((new_pos, state.vel)) {
                    let new_score = current_score + 1;
                    queue.push((-new_score, State::cons(new_pos, state.vel, new_score)));
                }
            }

            for new_dir in [state.vel.rotate_cw(), state.vel.rotate_ccw()] {
                if visited.insert((state.pos, new_dir)) {
                    let new_score = current_score + 1000;
                    queue.push((-new_score, State::cons(state.pos, new_dir, new_score)));
                }
            }
        }

        Int::MAX
    }

    fn find_tile(&self, tile: char) -> Option<Vec2<usize>>
    {
        (0..self.height).find_map(|y| {
            (0..self.width).find_map(|x| {
                if self.maze[y][x] == tile {
                    Some(Vec2::cons(x, y))
                } else {
                    None
                }
            })
        })
    }

    fn idx(&self, old: &Vec2<usize>, delta: &Vec2<isize>) -> Option<Vec2<usize>>
    {
        let nx = (old.x as isize + delta.x) as usize;
        let ny = (old.y as isize + delta.y) as usize;
        if nx < self.width && ny < self.height {
            Some(Vec2::cons(nx, ny))
        } else {
            None
        }
    }

    fn solve_two(&self) -> Int { 0 }
}



#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn rotate_testing()
    {
        let v1 = Vec2::cons(1, 0);
        let v2 = Vec2::cons(0, -1);

        assert!(v1.rotate_cw() == Vec2::cons(0, -1));
        assert!(v1.rotate_ccw() == Vec2::cons(0, 1));
        assert!(v2.rotate_cw() == Vec2::cons(-1, 0));
        assert!(v2.rotate_ccw() == Vec2::cons(1, 0));
    }
}
