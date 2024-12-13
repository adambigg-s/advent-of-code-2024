


use std::{
    env, fs, time::Instant,
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
    let buffer: String = fs::read_to_string(file_path).unwrap();
    if testing { println!("{buffer}"); }

    let s1: Instant = Instant::now();
    let solution: Solution = Solution::cons(&buffer);
    let part_one: Num = solution.solve_one();
    let p1: std::time::Duration = s1.elapsed();

    let s2: Instant = Instant::now();
    let solution: Solution = Solution::cons(&buffer);
    let part_two: Num = solution.solve_two();
    let p2: std::time::Duration = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}\n", part_one, p1);
    println!("part two: {}\ntime two: {:#?}\n", part_two, p2);
}

#[derive(Debug)]
struct GeoData
{
    area: Num, perimeter: Num, corners: Num,
    _tag: char,
}

impl GeoData
{
    fn cons(_tag: char) -> GeoData
    {
        GeoData { area: 0, perimeter: 0, corners: 0, _tag }
    }
}

#[derive(Debug)]
struct Solution
{
    grid: Vec<Vec<char>>,
    xdim: usize,
    ydim: usize,
}

impl Solution
{
    fn cons(buffer: &str) -> Solution
    {
        let grid: Vec<Vec<char>> = buffer.trim().lines().map(|line| {
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
                let tag = self.grid[y][x];
                let mut data: GeoData = GeoData::cons(tag);
                self.recursive_search_peri(x, y, tag, &mut seen, &mut data);
                if data.area != 0{
                    plots.push(data);
                }
            })
        });

        plots.iter().map(|plot| plot.area * plot.perimeter).sum()
    }

    fn solve_two(&self) -> Num
    {
        let mut seen: Vec<Vec<bool>> = vec![vec![false; self.xdim]; self.ydim];
        let mut plots: Vec<GeoData> = Vec::new();

        (0..self.ydim).for_each(|y| {
            (0..self.xdim).for_each(|x| {
                let tag = self.grid[y][x];
                let mut data: GeoData = GeoData::cons(tag);
                self.recursive_search_sides(x, y, tag, &mut seen, &mut data);
                if data.area != 0 {
                    plots.push(data);
                }
            })
        });

        plots.iter().map(|plot| plot.area * plot.corners).sum()
    }

    fn recursive_search_sides(&self, x: usize, y: usize, tag: char, seen: &mut Vec<Vec<bool>>, data: &mut GeoData)
    {
        if self.grid[y][x] != tag || seen[y][x] {
            return;
        }

        seen[y][x] = true;
        data.area += 1;
        data.corners += self.count_corners(x, y, tag);
        for (dx, dy) in get_directions() {
            if let Some((nx, ny)) = self.idx(x, y, dx, dy) {
                self.recursive_search_sides(nx, ny, tag, seen, data);
            }
        }
    }

    fn recursive_search_peri(&self, x: usize, y: usize, tag: char, seen: &mut Vec<Vec<bool>>, data: &mut GeoData)
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
                self.recursive_search_peri(nx, ny, tag, seen, data);
            } else {
                data.perimeter += 1;
            }
        }
    }

    fn count_corners(&self, x: usize, y: usize, tag: char) -> Num
    {
        let mut corners: Num = 0;

        for opt in get_corner_opts() {
            let diag_empty = self.idx(x, y, opt.diag.x, opt.diag.y).map_or(true, |(nx, ny)| {
                self.grid[ny][nx] != tag
            });
            let adj1 = self.idx(x, y, opt.adj1.x, opt.adj1.y).map_or(true, |(nx, ny)| {
                self.grid[ny][nx] != tag
            });
            let adj2 = self.idx(x, y, opt.adj2.x, opt.adj2.y).map_or(true, |(nx, ny)| {
                self.grid[ny][nx] != tag
            });

            if (diag_empty && adj1 == adj2) || (adj1 && adj2) {
                corners += 1;
            }
        }
        
        corners
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
}

struct Vec2
{
    x: isize, y: isize,
}

impl Vec2
{
    const fn cons(x: isize, y: isize) -> Vec2 { Vec2 { x, y } }
}

struct Neighbors
{
    diag: Vec2, adj1: Vec2, adj2: Vec2,
}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, 1), (0, -1),
    (1, 0), (-1, 0),
];

const CORNER_CHECKS: [Neighbors; 4] = [
    Neighbors { diag: Vec2::cons(1, 1), adj1: Vec2::cons(1, 0), adj2: Vec2::cons(0, 1) },
    Neighbors { diag: Vec2::cons(1, -1), adj1: Vec2::cons(1, 0), adj2: Vec2::cons(0, -1) },
    Neighbors { diag: Vec2::cons(-1, 1), adj1: Vec2::cons(-1, 0), adj2: Vec2::cons(0, 1) },
    Neighbors { diag: Vec2::cons(-1, -1), adj1: Vec2::cons(-1, 0), adj2: Vec2::cons(0, -1) },
];

fn get_directions() -> [(isize, isize); 4] { DIRECTIONS }

fn get_corner_opts() -> [Neighbors; 4] { CORNER_CHECKS }



#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn single_test()
    {
        let buffer: &str = "
XOO
OOO
OOO
        ";
        let solution: Solution = Solution::cons(buffer);
        let data: GeoData = GeoData::cons(solution.grid[0][0]);
        let cor = solution.count_corners(0, 0, data._tag);

        println!("{:?}, tag: {:?}", cor, data._tag);
        println!("({}, {})", solution.xdim, solution.ydim);
        assert!(cor == 4);
    }

    #[test]
    fn concave_test()
    {
        let buffer: &str = "
XOO
XXO
XXX
        ";
        let solution: Solution = Solution::cons(buffer);
        let data: GeoData = GeoData::cons(solution.grid[1][1]);
        let cor = solution.count_corners(1, 1, data._tag);

        println!("{:?}, tag: {:?}", cor, data._tag);
        println!("({}, {})", solution.xdim, solution.ydim);
        assert!(cor == 1);
    }

    #[test]
    fn convex_test()
    {
        let buffer: &str = "
XOO
XOO
XXX
        ";
        let solution: Solution = Solution::cons(buffer);
        let data: GeoData = GeoData::cons(solution.grid[1][1]);
        let cor = solution.count_corners(1, 1, data._tag);

        println!("{:?}, tag: {:?}", cor, data._tag);
        println!("({}, {})", solution.xdim, solution.ydim);
        assert!(cor == 1);
    }

    #[test]
    fn solid_test()
    {
        let buffer: &str = "
XXX
XXX
XXX
        ";
        let solution: Solution = Solution::cons(buffer);
        let data: GeoData = GeoData::cons(solution.grid[1][1]);
        let cor = solution.count_corners(1, 1, data._tag);

        println!("{:?}, tag: {:?}", cor, data._tag);
        println!("({}, {})", solution.xdim, solution.ydim);
        assert!(cor == 0);
    }

    #[test]
    fn small_test()
    {
        let buffer: &str = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
        ";
        let solution: Solution = Solution::cons(buffer);
        let x = solution.solve_two();

        println!("{}", x);
        println!("{:?}", solution);
        assert!(x == 236);
    }

    #[test]
    fn larger_test()
    {
        let buffer: &str = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
        ";
        let solution: Solution = Solution::cons(buffer);
        let x = solution.solve_two();

        println!("{}", x);
        println!("{}", buffer);
        assert!(x == 368);
    }
}
