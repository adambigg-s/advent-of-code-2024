


use std::{
    env, fs, time::Instant,
};



type Num = i128;

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
    let part_one: Num = solution.solve_one();
    let p1: std::time::Duration = s1.elapsed();

    let s2: Instant = Instant::now();
    let mut solution: Solution = Solution::construct(&buffer);
    let part_two: Num = solution.solve_two();
    let p2: std::time::Duration = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}\n", part_one, p1);
    println!("part two: {}\ntime two: {:#?}\n", part_two, p2);
}

#[derive(Debug)]
struct MatEq2
{
    mat: Vec<Vec<Num>>,
    vec: Vec<Num>,
}

impl MatEq2
{
    fn cons_list(a: Vec<Num>, vec: Vec<Num>) -> MatEq2
    {
        let mat = vec![
            vec![a[0], a[1]],
            vec![a[2], a[3]],
        ];

        MatEq2 { mat, vec }
    }

    fn cramer(&self) -> Option<Vec2<Num>>
    {
        let det_a = self.mat[0][0] * self.mat[1][1]  - self.mat[0][1] * self.mat[1][0];
        let det_ax = self.vec[0] * self.mat[1][1] - self.mat[0][1] * self.vec[1];
        let det_ay = self.mat[0][0] * self.vec[1] - self.vec[0] * self.mat[1][0];
        let x = det_ax / det_a;
        let y = det_ay / det_a;

        if self.solves(x, y) {
            Some(Vec2::cons(x, y))
        } else {
            None
        }
    }

    fn solves(&self, x: Num, y: Num) -> bool
    {
        self.mat[0][0] * x + self.mat[0][1] * y == self.vec[0]
            && self.mat[1][0] * x + self.mat[1][1] * y == self.vec[1]
    }
}

#[derive(Debug)]
struct Solution
{
    machines: Vec<MatEq2>,
}

impl Solution
{
    fn construct(buffer: &str) -> Solution
    {
        let mut machines = Vec::new();

        fn parse_vec(line: &str, delimiter: char, splitter: char) -> Vec2<Num>
        {
            let parts: Vec<&str> = line.split(delimiter).collect();
            let x: Num = parts[0].split(splitter).nth(1).unwrap().parse::<Num>().unwrap();
            let y: Num = parts[1].split(splitter).nth(1).unwrap().parse::<Num>().unwrap();

            Vec2::cons(x, y)
        }

        let lines: Vec<&str> = buffer.lines().collect();
        for chunk in lines.chunks(4) {
            let a: Vec2<Num> = parse_vec(chunk[0], ',', '+');
            let b: Vec2<Num> = parse_vec(chunk[1], ',', '+');
            let total: Vec2<Num> = parse_vec(chunk[2], ',', '=');

            let eqn = MatEq2::cons_list(
                vec![a.x, b.x, a.y, b.y],
                vec![total.x, total.y],
            );

            machines.push(eqn);
        }

        Solution { machines }
    }

    fn solve_one(&mut self) -> Num
    {
        let mut total: Num = 0;

        for machine in &self.machines {
            if let Some(vec) = machine.cramer() {
                total += 3 * vec.x + vec.y;
            }
        }

        total
    }

    fn solve_two(&mut self) -> Num
    {
        let mut total: Num = 0;
        let correction: Num = 10000000000000;

        for machine in &mut self.machines {
            machine.vec[0] += correction;
            machine.vec[1] += correction;
        }

        for machine in &self.machines {
            if let Some(vec) = machine.cramer() {
                total += 3 * vec.x + vec.y;
            }
        }

        total
    }
}

#[derive(Debug)]
struct Vec2<T>
{
    x: T, y: T
}

impl<T> Vec2<T>
{
    fn cons(x: T, y: T) -> Vec2<T> { Vec2 { x, y } }
}
