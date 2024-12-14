


use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

use std::{
    env, fs, thread::sleep, time::{Duration, Instant}
};



type Num = i32;
type Color = u32;

fn main()
{
    env::set_var("RUST_BACKTRACE", "full");
    let envs: Vec<String> = env::args().collect();
    let mut testing: bool = false;
    let file_path: &str = envs.get(1).map(|path| path.as_str()).unwrap_or_else(|| {
        testing = true;
        "testing.txt"
    });
    let file_buffer: String = fs::read_to_string(file_path).unwrap();
    if testing { println!("{file_buffer}"); }

    let s1: Instant = Instant::now();
    let mut solution: Solution = Solution::cons(&file_buffer);
    let part_one: Num = solution.solve_one();
    let p1: std::time::Duration = s1.elapsed();

    let s2: Instant = Instant::now();
    let mut solution: Solution = Solution::cons(&file_buffer);
    let part_two: Num = solution.solve_two();
    let p2: std::time::Duration = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}\n", part_one, p1);
    println!("part two: {}\ntime two: {:#?}\n", part_two, p2);
}

#[derive(Debug)]
struct RenderContext
{
    window: Window,
    buffer: Buffer,
}

impl RenderContext
{
    fn cons(width: usize, height: usize) -> RenderContext
    {
        let mut window: Window = Window::new(
            "aoc day 12 visulization",
            width,
            height,
            WindowOptions {
                scale: Scale::X8,
                scale_mode: ScaleMode::Stretch,
                ..WindowOptions::default()
            }
        ).unwrap();
        window.set_target_fps(60);
        let buffer: Buffer = Buffer::construct(width, height);

        RenderContext { window, buffer }
    }

    fn init_buffer(&mut self, grid: &[Vec<char>])
    {
        (0..self.buffer.height).for_each(|y| {
            (0..self.buffer.width).for_each(|x| {
                let data = darken(get_char_color(grid[y][x]), 0.5);
                self.buffer.place_pixel(x, y, data);
            })
        });
    }
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
    width: usize,
    height: usize,
    renderer: RenderContext,
}

impl Solution
{
    fn cons(file_buffer: &str) -> Solution
    {
        let grid: Vec<Vec<char>> = file_buffer.trim().lines().map(|line| {
            line.chars().collect()
        }).collect();
        let height = grid.len();
        let width = grid.first().unwrap().len();
        let mut renderer = RenderContext::cons(width, height);
        renderer.init_buffer(&grid);
        
        Solution { grid, width, height, renderer }
    }

    fn solve_one(&mut self) -> Num
    {
        let mut seen: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];
        let mut plots: Vec<GeoData> = Vec::new();

        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let tag = self.grid[y][x];
                let mut data: GeoData = GeoData::cons(tag);
                self.recursive_search_peri(x, y, tag, &mut seen, &mut data);
                if data.area != 0{
                    plots.push(data);
                }
                self.update_visuals(x, y);
            })
        });

        plots.iter().map(|plot| plot.area * plot.perimeter).sum()
    }

    fn update_visuals(&mut self, x: usize, y: usize)
    {
        self.renderer.buffer.place_pixel(x, y, 0xFF00FFFF);
        let buff = &self.renderer.buffer;
        self.renderer.window.update_with_buffer(&buff.pixels, buff.width, buff.height).unwrap();
        sleep(Duration::from_millis(10));
    }

    fn solve_two(&mut self) -> Num
    {
        let mut seen: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];
        let mut plots: Vec<GeoData> = Vec::new();

        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
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

        if nx < self.width && ny < self.height {
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

impl Neighbors
{
    const fn cons(diag: Vec2, adj1: Vec2, adj2: Vec2) -> Neighbors
    { Neighbors { diag, adj1, adj2 } }
}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, 1), (0, -1),
    (1, 0), (-1, 0),
];

const CORNER_CHECKS: [Neighbors; 4] = [
    Neighbors::cons(Vec2::cons(1, 1), Vec2::cons(1, 0), Vec2::cons(0, 1)),
    Neighbors::cons(Vec2::cons(-1, 1), Vec2::cons(-1, 0), Vec2::cons(0, 1)),
    Neighbors::cons(Vec2::cons(-1, 1), Vec2::cons(-1, 0), Vec2::cons(0, 1)),
    Neighbors::cons(Vec2::cons(-1, -1), Vec2::cons(-1, 0), Vec2::cons(0, -1)),
];

fn get_directions() -> [(isize, isize); 4] { DIRECTIONS }

fn get_corner_opts() -> [Neighbors; 4] { CORNER_CHECKS }

#[derive(Debug)]
struct Buffer
{
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Buffer
{
    fn construct(width: usize, height: usize) -> Buffer
    {
        Buffer { width, height, pixels: vec![0xFF000000; width * height] }
    }

    fn place_pixel(&mut self, x: usize, y: usize, data: Color)
    {
        {
            debug_assert!(x < self.width);
            debug_assert!(y < self.height)
        }
        self.pixels[self.width * y + x] = data;
    }
}

fn get_char_color(chr: char) -> Color
{
    match chr.to_ascii_lowercase() {
        'a' => 0xFFFFC0CB, // Pastel Pink
        'b' => 0xFFADD8E6, // Pastel Blue
        'c' => 0xFF98FB98, // Pastel Green
        'd' => 0xFFFFD700, // Pastel Yellow
        'e' => 0xFFFFB6C1, // Light Pink
        'f' => 0xFFDA70D6, // Orchid
        'g' => 0xFF87CEFA, // Light Sky Blue
        'h' => 0xFFFFA07A, // Light Salmon
        'i' => 0xFFE6E6FA, // Lavender
        'j' => 0xFFAFEEEE, // Pale Turquoise
        'k' => 0xFFF5DEB3, // Wheat
        'l' => 0xFFFFE4E1, // Misty Rose
        'm' => 0xFFD8BFD8, // Thistle
        'n' => 0xFFB0E0E6, // Powder Blue
        'o' => 0xFFFFEFD5, // Papaya Whip
        'p' => 0xFFFFDAB9, // Peach Puff
        'q' => 0xFFFAFAD2, // Light Goldenrod Yellow
        'r' => 0xFFE0FFFF, // Light Cyan
        's' => 0xFF7FFFD4, // Aquamarine
        't' => 0xFFFFFACD, // Lemon Chiffon
        'u' => 0xFFFFA500, // Pastel Orange
        'v' => 0xFFEE82EE, // Violet
        'w' => 0xFFDEB887, // Burlywood
        'x' => 0xFFE9967A, // Dark Salmon
        'y' => 0xFFFA8072, // Salmon
        'z' => 0xFF90EE90, // Light Green
        _ => 0xFF000000, // Default Black for unrecognized characters
    }
}

fn darken(col: Color, factor: f32) -> Color
{
    let (r, g, b) = to_rgb(col);
    let factor = factor.clamp(0.0, 1.0);
    let r = (r as f32 * factor) as u8;
    let g = (g as f32 * factor) as u8;
    let b = (b as f32 * factor) as u8;
    from_u8(r, g, b)
}

fn to_rgb(col: Color) -> (u8, u8 ,u8)
{
    let r = ((col >> 16) & 0xFF) as u8;
    let g = ((col >> 8) & 0xFF) as u8;
    let b = (col & 0xFF) as u8;
    (r, g, b)
}

fn from_u8(r: u8, g: u8, b: u8) -> Color
{
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
