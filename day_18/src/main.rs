


use std::{collections::{HashSet, VecDeque}, env, fs, time::Instant};



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
    let solution: Solution = Solution::construct(&buffer, testing);
    let part_one: Int = solution.solve_one();
    let p1: std::time::Duration = s1.elapsed();

    let s2: Instant = Instant::now();
    let mut solution: Solution = Solution::construct(&buffer, testing);
    let part_two: String = solution.solve_two(&buffer, testing);
    let p2: std::time::Duration = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}\n", part_one, p1);
    println!("part two: {}\ntime two: {:#?}\n", part_two, p2);
}

struct Solution
{
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Solution
{
    fn construct(buffer: &str, testing: bool) -> Solution
    {
        let dim: usize = if testing { 7 } else { 71 };
        let fallen = if testing { 12 } else { 1024 };
        let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Empty; dim]; dim];
        buffer.trim().lines().enumerate().for_each(|(byte, line)| {
            let mut coords = line.split(',');
            let (x, y) = (coords.next().unwrap().parse::<usize>().unwrap(), coords.next().unwrap().parse::<usize>().unwrap());
            if byte < fallen {
                grid[y][x] = Cell::Byte;
            }
        });
        let width = grid.first().unwrap().len();
        let height = grid.len();
        grid[0][0] = Cell::Start;
        grid[height-1][width-1] = Cell::End;

        Solution { grid, width, height }
    }

    fn solve_one(&self) -> Int
    {
        self.bread_fish()
    }

    fn solve_two(&mut self, buffer: &str, testing: bool) -> String
    {
        let coords: Vec<(usize, usize)> = buffer.trim().lines().map(|line| {
            let mut coords = line.split(',');
            (coords.next().unwrap().parse::<usize>().unwrap(), coords.next().unwrap().parse::<usize>().unwrap())
        }).collect();

        let mut counter = if testing { 12 } else { 1024 };
        while counter < coords.len() {
            let (x, y) = coords[counter];
            self.grid[y][x] = Cell::Byte;
            if self.bread_fish() == Int::MAX {
                return format!("{},{}", x, y);
            }
            counter += 1;
        }
        "none found".to_string()
    }

    fn bread_fish(&self) -> Int
    {
        let mut seen: HashSet<Position> = HashSet::new();
        let mut queue: VecDeque<State> = VecDeque::new();

        let init = State::cons(0, 0, 0, 0, 0);
        queue.push_back(init);
        seen.insert(init.position);

        while let Some(state) = queue.pop_front() {
            if self.grid[state.position.y][state.position.x].is_end() {
                return state.steps;
            }

            for dir in get_directions() {
                if let Some((nx, ny)) = self.idx(state.position.x, state.position.y, dir.0, dir.1) {
                    let new_pos = Position { x: nx, y: ny, dx: dir.0, dy: dir.1 };
                    if !self.grid[ny][nx].is_byte() && seen.insert(new_pos) {
                        queue.push_back(State::cons(nx, ny, dir.0, dir.1, state.steps + 1));
                    }
                }
            }
        }
        Int::MAX
    }

    fn idx(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)>
    {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;
        if nx < self.width && ny < self.height {
            Some((nx, ny))
        } else {
            None
        }
    }
}

#[derive(Hash, PartialEq, PartialOrd, Clone, Copy, Eq)]
struct Position
{
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}

#[derive(Clone, Copy)]
struct State
{
    position: Position,
    steps: Int,
}

impl State
{
    fn cons(x: usize, y: usize, dx: isize, dy: isize, steps: Int) -> State
    {
        State { position: Position { x, y, dx, dy }, steps }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell
{
    Empty,
    Byte,
    Start,
    End,
}

impl Cell
{
    fn is_end(&self) -> bool
    {
        *self == Cell::End
    }

    fn is_byte(&self) -> bool
    {
        *self == Cell::Byte
    }
}

fn get_directions() -> [(isize, isize); 4]
{
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
}
