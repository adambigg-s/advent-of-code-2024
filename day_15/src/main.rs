


use std::{
    collections::HashSet, env, fs, time::Instant
};



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
    let mut solution: Solution = Solution::construct(&buffer, testing);
    let part_one: Int = solution.solve_one();
    let p1: std::time::Duration = s1.elapsed();

    let s2: Instant = Instant::now();
    let mut solution: Solution = Solution::construct(&buffer, testing);
    let part_two: Int = solution.solve_two();
    let p2: std::time::Duration = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}\n", part_one, p1);
    println!("part two: {}\ntime two: {:#?}\n", part_two, p2);
}

#[derive(Debug)]
struct Solution
{
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    robot_coords: Vec2<usize>,
    instructions: Vec<char>,
}

impl Solution
{
    fn construct(buffer: &str, testing: bool) -> Solution
    {
        let mut parts = buffer.trim().split("\r\n\r\n");
        if !testing {
            parts = buffer.trim().split("\n\n");
        }
        let grid = parts.next().unwrap();
        let instructions = parts.next().unwrap();
        let grid: Vec<Vec<char>> = grid.lines().map(|line| {
            line.chars().collect()
        }).collect();
        let width = grid.first().unwrap().len();
        let height = grid.len();
        let instructions = instructions.chars().collect();
        let robot_coords = Self::find_robot(&grid, width, height).unwrap();

        Solution { grid, robot_coords, width, height, instructions }
    }

    fn solve_one(&mut self) -> Int
    {
        for idx in 0..self.instructions.len() {
            let movement = self.instructions[idx].get_movement();
            if let Some(destination) = self.idx(&self.robot_coords, &movement) {
                self.recur_push(&destination, &movement);
                self.try_move(&destination);
            }
        }

        self.coordinate_sum()
    }

    fn solve_two(&mut self) -> Int
    {
        self.doublify();
        for idx in 0..self.instructions.len() {
            let movement = self.instructions[idx].get_movement();
            if let Some(destination) = self.idx(&self.robot_coords, &movement) {
                if self.can_push_move(&destination, &movement) {
                    self.recur_push_big(&destination, &movement);
                }
                self.try_move(&destination);
            }
        }

        self.coordinate_sum()
    }

    fn can_push_move(&self, start: &Vec2<usize>, direction: &Vec2<isize>) -> bool
    {
        let mut coords: Vec<Vec2<usize>> = Vec::new();
        let mut visited: HashSet<Vec2<usize>> = HashSet::new();
        self.recur_push_coords(direction, start, &mut coords, &mut visited);

        coords.iter().all(|coord| {
            let new = self.idx(coord, direction).unwrap();
            self.grid[new.y][new.x].is_empty() || coords.contains(&new)
        })
    }

    fn recur_push_coords(
        &self,
        direction: &Vec2<isize>,
        start: &Vec2<usize>,
        coord: &mut Vec<Vec2<usize>>,
        visited: &mut HashSet<Vec2<usize>>
    )
    {
        if !self.grid[start.y][start.x].is_box_any() || visited.contains(start) {
            return;
        }

        visited.insert(*start);
        let doublet = self.big_box_coords(start);
        for &box_coord in &doublet {
            if !coord.contains(&box_coord) {
                coord.push(box_coord);
            }

            if let Some(next) = self.idx(&box_coord, direction) {
                self.recur_push_coords(direction, &next, coord, visited);
            }
        }
    }

    fn recur_push_big(&mut self, start: &Vec2<usize>, direction: &Vec2<isize>)
    {
        if !self.grid[start.y][start.x].is_box_any() {
            return;
        }
        let doublet = self.big_box_coords(start);
        if let (Some(le), Some(ri)) = {
            (self.idx(&doublet[0], direction), self.idx(&doublet[1], direction))
        }
        {
            if self.grid[le.y][le.x].is_wall() || self.grid[ri.y][ri.x].is_wall() {
                return;
            }
            if self.grid[le.y][le.x].is_box_any() && doublet.contains(&ri) {
                self.recur_push_big(&le, direction);
            }
            if self.grid[ri.y][ri.x].is_box_any() && doublet.contains(&le) {
                self.recur_push_big(&ri, direction);
            }
            if self.grid[le.y][le.x].is_box_any() && !doublet.contains(&le) {
                self.recur_push_big(&le, direction);
            }
            if self.grid[ri.y][ri.x].is_box_any() && !doublet.contains(&ri) {
                self.recur_push_big(&ri, direction);
            }

            if self.grid[le.y][le.x].is_empty() && doublet.contains(&ri) {
                self.swap(doublet[0], le);
                self.swap(doublet[1], ri);
            }
            else if self.grid[ri.y][ri.x].is_empty() && doublet.contains(&le) {
                self.swap(doublet[1], ri);
                self.swap(doublet[0], le);
            }
            else if self.grid[le.y][le.x].is_empty() && self.grid[ri.y][ri.x].is_empty() {
                self.swap(doublet[0], le);
                self.swap(doublet[1], ri);
            }
        }
    }

    fn recur_push(&mut self, start: &Vec2<usize>, direction: &Vec2<isize>)
    {
        if !self.grid[start.y][start.x].is_box() {
            return;
        }
        if let Some(target) = self.idx(start, direction) {
            if self.grid[target.y][target.x].is_wall() {
                return;
            }
            if self.grid[target.y][target.x].is_box() {
                self.recur_push(&target, direction);
            }

            if self.grid[target.y][target.x].is_empty() {
                self.swap(*start, target);
            }
        }
    }

    fn big_box_coords(&self, start: &Vec2<usize>) -> [Vec2<usize>; 2]
    {
        if self.grid[start.y][start.x].is_box_left() {
            [*start, Vec2::cons(start.x + 1, start.y)]
        }
        else if self.grid[start.y][start.x].is_box_right() {
            [Vec2::cons(start.x - 1, start.y), *start]
        }
        else {
            panic!()
        }
    }

    fn try_move(&mut self, target: &Vec2<usize>) -> bool
    {
        if self.grid[target.y][target.x].is_empty() {
            self.swap(self.robot_coords, *target);
            self.robot_coords = *target;
            return true;
        }
        false
    }

    #[allow(dead_code)]
    fn print_grid(&self)
    {
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                print!("{}", self.grid[y][x]);
            });
            println!();
        });
    }

    fn doublify(&mut self)
    {
        for y in 0..self.height {
            let mut nx = Vec::new();
            for x in 0..self.width {
                let entity = self.grid[y][x];
                if entity.is_wall() {
                    nx.push(wall_entity());
                    nx.push(wall_entity());
                }
                else if entity.is_empty() || entity.is_robot() {
                    nx.push(empty_entity());
                    nx.push(empty_entity());
                }
                else if entity.is_box() {
                    nx.push(box_left());
                    nx.push(box_right());
                }
            }
            self.grid[y] = nx;
        }
        self.width *= 2;
        self.robot_coords.x *= 2;
        self.grid[self.robot_coords.y][self.robot_coords.x] = robot_entity();
    }

    fn coordinate_sum(&self) -> Int
    {
        let mut result: Int = 0;
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                if self.grid[y][x].is_box() || self.grid[y][x].is_box_left() {
                    result += (100 * y + x) as Int
                }
            })
        });

        result
    }

    fn swap(&mut self, from: Vec2<usize>, to: Vec2<usize>)
    {
        let temp = self.grid[from.y][from.x];
        self.grid[from.y][from.x] = self.grid[to.y][to.x];
        self.grid[to.y][to.x] = temp;
    }

    fn find_robot(grid: &[Vec<char>], width: usize, height: usize) -> Option<Vec2<usize>>
    {
        (0..height).find_map(|y| {
            (0..width).find_map(|x| {
                if grid[y][x].is_robot() {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2<T> { x: T, y: T, }

impl<T> Vec2<T> { fn cons(x: T, y: T) -> Vec2<T> { Vec2 { x, y} } }

trait CharEntity
{
    fn is_wall(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_box(&self) -> bool;
    fn is_robot(&self) -> bool;
    fn is_up(&self) -> bool;
    fn is_down(&self) -> bool;
    fn is_left(&self) -> bool;
    fn is_right(&self) -> bool;
    fn is_box_any(&self) -> bool;
    fn is_box_left(&self) -> bool;
    fn is_box_right(&self) -> bool;
    fn get_movement(&self) -> Vec2<isize>
    {
        if self.is_up() {
            Vec2::cons(0, -1)
        } else if self.is_down() {
            Vec2::cons(0, 1)
        } else if self.is_left() {
            Vec2::cons(-1, 0)
        } else if self.is_right() {
            Vec2::cons(1, 0)
        } else {
            Vec2::cons(0, 0)
        }
    }
}

impl CharEntity for char
{
    fn is_wall(&self) -> bool { *self == '#' }
    fn is_empty(&self) -> bool { *self == '.' }
    fn is_box(&self) -> bool { *self == 'O' }
    fn is_robot(&self) -> bool { *self == '@' }
    fn is_up(&self) -> bool { *self == '^' }
    fn is_down(&self) -> bool { *self == 'v' }
    fn is_left(&self) -> bool { *self == '<' }
    fn is_right(&self) -> bool { *self == '>' }
    fn is_box_any(&self) -> bool { *self == '[' || *self == ']' }
    fn is_box_left(&self) -> bool { *self == '[' }
    fn is_box_right(&self) -> bool { *self == ']' }
}

fn empty_entity() -> char { '.' }
fn wall_entity() -> char { '#' }
fn box_left() -> char { '[' }
fn box_right() -> char { ']' }
fn robot_entity() -> char { '@' }
