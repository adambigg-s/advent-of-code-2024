


use std::{collections::{BinaryHeap, HashMap, HashSet}, env, fs, time::Instant};

use day_16::*;

use aoc_library::*;



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
        self.bread_fish()
    }

    fn solve_two(&self) -> Int
    {
        self.bread_fish_path()
    }

    fn bread_fish(&self) -> Int
    {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        let mut visited: HashSet<State> = HashSet::new();

        let start = self.find_tile(start()).unwrap();
        let state = State::cons(start, Vec2::cons(1, 0), 0);
        heap.push(state);
        visited.insert(state);

        while let Some(state) = heap.pop() {
            if self.maze[state.pos.y][state.pos.x].is_end() {
                return -state.score;
            }

            for dir in [state.vel.rotate_cw(), state.vel.rotate_ccw()] {
                let new_state = State::cons(state.pos, dir, state.score - 1000);
                if visited.insert(new_state) {
                    heap.push(new_state)
                }
            }

            if let Some(new_pos) = self.idx(&state.pos, &state.vel) {
                if !self.maze[new_pos.y][new_pos.x].is_wall() {
                    let new_state = State::cons(new_pos, state.vel, state.score - 1);
                    if visited.insert(new_state) {
                        heap.push(new_state);
                    }
                }
            }
        }
        Int::MAX
    }

    fn bread_fish_path(&self) -> Int
    {
        0
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

    #[test]
    fn heap_testing()
    {
        let state1 = State::cons(Vec2::cons(1, 1), Vec2::cons(0, 1), 1);
        let state2 = State::cons(Vec2::cons(1, 1), Vec2::cons(1, -1), 10);
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        heap.push(state1);
        heap.push(state2);

        assert!(heap.pop().unwrap() == state2);
        assert!(heap.pop().unwrap() == state1);

        let state1 = State::cons(Vec2::cons(1, 1), Vec2::cons(0, 1), -1);
        let state2 = State::cons(Vec2::cons(1, 1), Vec2::cons(1, -1), -10);
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        heap.push(state1);
        heap.push(state2);

        assert!(heap.pop().unwrap() == state1);
        assert!(heap.pop().unwrap() == state2);
    }
}
