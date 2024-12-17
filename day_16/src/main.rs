


use std::{collections::VecDeque, env, fs, time::Instant};



type Int = i32;

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
        let start = self.find_tile(start()).unwrap();
        let end = self.find_tile(end()).unwrap();

        self.bread_fish_church(start, end)
    }

    fn bread_fish_church(&self, start: Vec2<usize>, end: Vec2<usize>) -> Int
    {
        let mut queue: VecDeque<State> = VecDeque::new();

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

    fn solve_two(&self) -> Int { 0 }
}

trait CharEntity
{
    fn is_start(&self) -> bool;
    fn is_end(&self) -> bool;
    fn is_wall(&self) -> bool;
    fn is_empty(&self) -> bool;
}

impl CharEntity for char
{
    fn is_start(&self) -> bool { *self == start() }
    fn is_end(&self) -> bool { *self == end() }
    fn is_wall(&self) -> bool { *self == wall() }
    fn is_empty(&self) -> bool { *self == empty() }
}

fn start() -> char { 'S' }
fn end() -> char { 'E' }
fn wall() -> char { '#' }
fn empty() -> char { ',' }

const DIRECTIONS: [Vec2<isize>; 4] = [
    Vec2::cons(0, 1), Vec2::cons(0, -1),
    Vec2::cons(1, 0), Vec2::cons(-1, 0),
];

fn get_directions() -> [Vec2<isize>; 4] { DIRECTIONS }

struct Vec2<T> { x: T, y: T, }

impl<T> Vec2<T> { const fn cons(x: T, y: T) -> Vec2<T> { Vec2 { x, y } } }

struct State
{
    pos: Vec2<usize>,
    direc: Vec2<isize>,
    score: Int,
}

impl State
{
    fn cons(pos: Vec2<usize>, direc: Vec2<isize>) -> State
    {
        State { pos, direc, score: 0 }
    }
}
