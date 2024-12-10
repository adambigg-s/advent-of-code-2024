


use std::{
    collections::{HashMap, HashSet}, 
    env, fs, 
    ops::{Add, Mul, Sub}, 
    process, time::Instant
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
    let buffer: String = fs::read_to_string(file_path).unwrap_or_else(|err| {
        println!("buffer read error: {err}");
        process::exit(3);
    });
    if testing {
        println!("{buffer}");
    }

    let solution: Solution = Solution::construct(&buffer);

    let s1 = Instant::now();
    let part_one = solution.solve_one();
    let p1 = s1.elapsed();

    let s2 = Instant::now();
    let part_two = solution.solve_two();
    let p2 = s2.elapsed();

    println!("part one: {}\ntime one: {:#?}", part_one, p1);
    println!();
    println!("part two: {}\ntime two: {:#?}", part_two, p2);
    println!();
}

struct Solution 
{
    grid: Vec<Vec<char>>,
    xdim: usize,
    ydim: usize,
}

impl Solution {
    fn construct(buffer: &str) -> Solution 
    {
        let grid: Vec<Vec<char>> = buffer.lines().map(|line| line.chars().collect()).collect();
        let xdim = grid.first().unwrap().len();
        let ydim = grid.len();

        Solution { grid, xdim, ydim }
    }

    fn solve_one(&self) -> Num 
    {
        let antennas: HashMap<char, Vec<Antenna>> = self.serialize_antennas();
        let antinodes: HashSet<Vec2<isize>> = self.find_antinodes(&antennas);

        antinodes
            .into_iter()
            .filter(|node| self.validate_antinode(node))
            .count() as Num
    }

    fn solve_two(&self) -> Num 
    {
        let antennas: HashMap<char, Vec<Antenna>> = self.serialize_antennas();
        let antinodes: HashSet<Vec2<isize>> = self.find_antinodes_resonance(&antennas);

        antinodes.len() as Num
    }

    fn validate_antinode(&self, antinode: &Vec2<isize>) -> bool 
    {
        (antinode.x as usize) < self.xdim && (antinode.y as usize) < self.ydim
    }

    fn find_antinodes_resonance(
        &self,
        antennas: &HashMap<char, Vec<Antenna>>,
    ) -> HashSet<Vec2<isize>> 
    {
        let mut antinodes: HashSet<Vec2<isize>> = HashSet::new();

        for tag in antennas.values() {
            for i in 0..(tag.len() - 1) {
                for j in (i + 1)..tag.len() {
                    let ant1 = &tag[i];
                    let ant2 = &tag[j];
                    let invslope: Vec2<isize> = Vec2::con(
                        ant1.coords.x - ant2.coords.x,
                        ant1.coords.y - ant2.coords.y,
                    );

                    let mut multiplier: isize = 0;
                    loop {
                        let antinode1 = ant1.coords + invslope * multiplier;
                        let antinode2 = ant2.coords - invslope * multiplier;

                        if !self.validate_antinode(&antinode1)
                            && !self.validate_antinode(&antinode2)
                        {
                            break;
                        }
                        if self.validate_antinode(&antinode1) {
                            antinodes.insert(antinode1);
                        }
                        if self.validate_antinode(&antinode2) {
                            antinodes.insert(antinode2);
                        }
                        multiplier += 1;
                    }
                }
            }
        }

        antinodes
    }

    fn find_antinodes(&self, antennas: &HashMap<char, Vec<Antenna>>) -> HashSet<Vec2<isize>> 
    {
        let mut antinodes: HashSet<Vec2<isize>> = HashSet::new();
        for tag in antennas.values() {
            for i in 0..(tag.len() - 1) {
                for j in (i + 1)..tag.len() {
                    let ant1 = &tag[i];
                    let ant2 = &tag[j];
                    let invslope: Vec2<isize> = Vec2::con(
                        ant1.coords.x - ant2.coords.x,
                        ant1.coords.y - ant2.coords.y,
                    );
                    antinodes.insert(ant1.coords + invslope);
                    antinodes.insert(ant2.coords - invslope);
                }
            }
        }

        antinodes
    }

    fn serialize_antennas(&self) -> HashMap<char, Vec<Antenna>> 
    {
        let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
        for y in 0..self.ydim {
            for x in 0..self.xdim {
                if !self.grid[y][x].is_empty() {
                    let identifier: char = self.grid[y][x];
                    let coords: Vec2<isize> = Vec2::con(x as isize, y as isize);

                    let antenna: Antenna = Antenna::con(identifier, coords);
                    antennas.entry(identifier).or_default().push(antenna);
                }
            }
        }

        antennas
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vec2<T> 
{
    x: T,
    y: T,
}

impl<T> Vec2<T>
{
    fn con(x: T, y: T) -> Vec2<T> 
    {
        Vec2 { x, y }
    }
}

impl Mul<isize> for Vec2<isize> 
{
    type Output = Vec2<isize>;

    fn mul(self, rhs: isize) -> Self::Output 
    {
        Vec2::con(
            self.x * rhs,
            self.y * rhs,
        )
    }
}

impl Add for Vec2<isize> 
{
    type Output = Vec2<isize>;

    fn add(self, rhs: Vec2<isize>) -> Self::Output
    {
        Vec2::con(
            self.x + rhs.x,
            self.y + rhs.y,
        )
    }
}

impl Sub for Vec2<isize>
{
    type Output = Vec2<isize>;

    fn sub(self, rhs: Vec2<isize>) -> Self::Output
    {
        Vec2::con(
            self.x - rhs.x,
            self.y - rhs.y
        )
    }
}

#[derive(Debug)]
struct Antenna 
{
    _tag: char,
    coords: Vec2<isize>,
}

impl Antenna 
{
    fn con(_tag: char, coords: Vec2<isize>) -> Antenna 
    {
        Antenna { _tag, coords }
    }
}

trait IsEmpty 
{
    fn is_empty(&self) -> bool;
}

impl IsEmpty for char 
{
    fn is_empty(&self) -> bool 
    {
        *self == '.'
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_test() 
    {
        assert!('.'.is_empty())
    }
}
