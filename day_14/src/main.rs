


use std::{env, fs, ops::{Add, AddAssign, Mul}, thread::sleep, time::{Duration, Instant}
};

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};



type Int = i32;
type Color = u32;

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
    let mut solution: Solution = Solution::construct(&buffer);
    let part_one: Int = solution.solve_one();
    let p1: std::time::Duration = s1.elapsed();

    let s2: Instant = Instant::now();
    let mut solution: Solution = Solution::construct(&buffer);
    let part_two: Int = solution.solve_two();
    let p2: std::time::Duration = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}\n", part_one, p1);
    println!("part two: {}\ntime two: {:#?}\n", part_two, p2);
}

impl Solution
{
    fn construct(buffer: &str) -> Solution
    {
        let robots = buffer.lines().filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let parse_vec2 = |str: &str| -> Option<Vec2> {
                let coords: Vec<&str> = str[2..].split(',').collect();
                Some(Vec2::cons(coords[0].parse::<Int>().ok()?, coords[1].parse::<Int>().ok()?))
            };
            let pos = parse_vec2(parts[0])?;
            let vel = parse_vec2(parts[1])?;
            Some(Robot::cons(pos, vel))
        }).collect();

        Solution { robots }
    }

    fn solve_one(&mut self) -> Int
    {
        let width = 101;
        let height = 103;
        let dt = 100;

        self.robots.iter_mut().for_each(|robot| {
            robot.patrol(dt);
            robot.confine_pos(width, height);
        });

        let (i, ii, iii, iv) = self.quad_count(width, height);
        
        i * ii * iii * iv
    }

    fn solve_two(&mut self) -> Int
    {
        let width = 101;
        let height = 103;
        let dt = 1;
        let mut window: Window = Window::new(
            "day 14 tree?? drawing",
            width,
            height,
            WindowOptions {
                scale: Scale::X8,
                scale_mode: ScaleMode::Stretch,
                ..WindowOptions::default()
            }
        ).unwrap();
        window.set_target_fps(30);
        let mut buffer: Buffer = Buffer::construct(width, height);
        buffer.place_pixel(10, 10, 0xFF00FFFF);

        let mut seconds: i32 = 6000;
        self.robots.iter_mut().for_each(|robot| {
            robot.patrol(seconds);
            robot.confine_pos(width as Int, height as Int);
        });

        while window.is_open() && !window.is_key_down(Key::Escape) {
            if window.is_key_down(Key::Space) {
                self.robots.iter_mut().for_each(|robot| {
                    robot.patrol(dt);
                    robot.confine_pos(width as Int, height as Int);
                });
                seconds += dt;
                sleep(Duration::from_millis(30));
            }
            if window.is_key_down(Key::B) {
                self.robots.iter_mut().for_each(|robot| {
                    robot.anti_patrol(dt);
                    robot.confine_pos(width as Int, height as Int);
                });
                seconds -= dt;
                sleep(Duration::from_millis(30));
            }
            self.update_visuals(&mut buffer);
            window.update_with_buffer(&buffer.pixels, buffer.width, buffer.height).unwrap();
        }

        seconds
    }

    fn update_visuals(&self, buffer: &mut Buffer)
    {
        buffer.clear();
        self.robots.iter().for_each(|robot| {
            let (x, y) = (robot.pos.x, robot.pos.y);
            let color: Color = 0xFF22FF44;
            buffer.place_pixel(x as usize, y as usize, color);
        });
    }

    fn quad_count(&self, width: Int, height: Int) -> (Int, Int, Int, Int)
    {
        let xmid = width / 2;
        let ymid = height / 2;

        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
        self.robots.iter().for_each(|robot| {
           if (xmid+1..width).contains(&robot.pos.x) && (ymid+1..height).contains(&robot.pos.y) {
               q1 += 1;
           }
           if (0..xmid).contains(&robot.pos.x) && (ymid+1..height).contains(&robot.pos.y) {
               q2 += 1;
           }
           if (0..xmid).contains(&robot.pos.x) && (0..ymid).contains(&robot.pos.y) {
               q3 += 1;
           }
           if (xmid+1..width).contains(&robot.pos.x) && (0..ymid).contains(&robot.pos.y) {
               q4 += 1;
           }
        });

        (q1, q2, q3, q4)
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec2
{
    x: Int, y: Int,
}

impl Vec2
{
    fn cons(x: Int, y: Int) -> Vec2 { Vec2 { x, y } }
}

impl Mul<Int> for Vec2
{
    type Output = Self;

    fn mul(self, rhs: Int) -> Self::Output
    {
        let (nx, ny) = (self.x * rhs, self.y * rhs);
        Vec2 { x: nx, y: ny }
    }
}

impl Add for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output
    {
        let (nx, ny) = (self.x + rhs.x, self.y + rhs.y);
        Vec2 { x: nx, y: ny }
    }
}

impl AddAssign for Vec2
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug)]
struct Robot
{
    pos: Vec2, vel: Vec2,
}

impl Robot
{
    fn cons(pos: Vec2, vel: Vec2) -> Robot
    {
        Robot { pos, vel }
    }

    fn patrol(&mut self, dt: Int)
    {
        self.pos += self.vel * dt;
    }

    fn anti_patrol(&mut self, dt: Int)
    {
        self.pos += self.vel * -dt;
    }

    fn confine_pos(&mut self, width: Int, height: Int)
    {
        self.pos.x %= width;
        self.pos.y %= height;

        if self.pos.x < 0 {
            self.pos.x += width;
        }
        if self.pos.y < 0 {
            self.pos.y += height;
        }
    }
}

#[derive(Debug)]
struct Solution
{
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Buffer
{
    pixels: Vec<Color>, width: usize, height: usize,
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
        let ty = self.height-1 - y;
        self.pixels[self.width * ty + x] = data;
    }

    fn clear(&mut self)
    {
        self.pixels.iter_mut().for_each(|pix| {
            *pix = 0xFFAAAAAA;
        });
    }
}

