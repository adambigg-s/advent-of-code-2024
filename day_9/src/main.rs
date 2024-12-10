


#[allow(unused_imports)]
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Div, Sub},
    env, fs, process,
    time::Instant,
};



type Num = i64;

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
    if testing { println!("{buffer}"); }

    let s1 = Instant::now();
    let mut solution: Solution = Solution::construct(&buffer);
    let part_one = solution.solve_one();
    let p1 = s1.elapsed();

    let s2 = Instant::now();
    let mut solution: Solution = Solution::construct(&buffer);
    let part_two = solution.solve_two();
    let p2 = s2.elapsed();

    println!("\n__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:#?}", part_one, p1);
    println!();
    println!("part two: {}\ntime two: {:#?}", part_two, p2);
    println!();
}

type ID = Num;

struct Solution
{
    disc: Vec<Option<ID>>,
    capacity: usize,
}

impl Solution
{
    fn construct(buffer: &str) -> Solution
    {
        let mut disc: Vec<Option<ID>> = Vec::new();
        let mut identifier: ID = 0;
        for (idx, chr) in buffer.trim().chars().enumerate() {
            if idx.is_even() {
                let reps = chr.to_digit(10).unwrap();
                (0..reps).for_each(|_| {
                    disc.push(Some(identifier));
                });
                identifier += 1;
            } else {
                let reps = chr.to_digit(10).unwrap();
                (0..reps).for_each(|_| {
                    disc.push(None);
                });
            }
        }
        let capacity = disc.len();
        
        Solution { disc, capacity }
    }

    fn solve_one(&mut self) -> Num
    {
        let mut lhs: usize = 0;
        let mut rhs: usize = self.capacity-1;

        loop {
            self.find_free(&mut lhs);
            self.find_data(&mut rhs);
            if lhs < rhs {
                self.disc[lhs] = self.disc[rhs];
                self.disc[rhs] = None;
                continue;
            }
            break;
        }

        self.checksum()
    }

    fn solve_two(&mut self) -> Num
    {
        let mut rhs: usize = self.capacity-1;
        loop {
            let taken = self.find_data_block(rhs);
            if let Some(free) = self.find_free_block(taken.size) {
                if taken.start > free.start {
                    self.swap(free.start, taken.start, taken.size);
                }
            }
            if taken.start == 0 {
                break;
            } else {
                rhs = taken.start - 1;
            }
        }

        self.checksum()
    }

    fn swap(&mut self, free_start: usize, taken_start: usize, size: usize)
    {
        for i in 0..size {
            self.disc[free_start + i] = self.disc[taken_start + i];
            self.disc[taken_start + i] = None;
        }
    }

    fn find_data_block(&self, before: usize) -> MemBlock
    {
        let mut end: usize = before;
        let id = self.disc[end];
        self.find_data(&mut end);
        let mut start = end;
        self.find_data_start(&mut start);
        let size = end - start + 1;

        MemBlock::construct(size, id, start)
    }

    fn find_free_block(&self, size_req: usize) -> Option<MemBlock>
    {
        let mut start: usize = 0;
        loop {
            self.find_free(&mut start);
            let mut end = start;
            self.find_free_end(&mut end);
            let size = end - start + 1;

            if size >= size_req {
                return Some(MemBlock::construct(size, None, start));
            } else {
                start = end + 1;
            }

            if start >= self.capacity-1 {
                break;
            }
        }

        None
    }

    #[allow(dead_code)]
    fn print_disc(&self)
    {
        for i in 0..self.capacity {
            match self.disc[i] {
                Some(id) => print!("{}", id),
                None => print!("."),
            }
        }
        println!();
    }

    fn checksum(&self) -> Num
    {
        self.disc
            .iter()
            .enumerate()
            .map(|(idx, id)| match id {
                Some(id) => (idx as ID) * id,
                None => 0,
            })
            .sum()
    }

    fn find_free(&self, curr: &mut usize)
    {
        while *curr < self.capacity && self.disc[*curr].is_some() {
            if *curr >= self.capacity-1 {
                break;
            }
            *curr += 1;
        }
    }

    fn find_free_end(&self, curr: &mut usize)
    {
        while *curr < self.capacity && self.disc[*curr].is_none() {
            if (*curr + 1) < self.capacity && self.disc[*curr + 1].is_some() {
                break;
            }
            *curr += 1;
        }
    }

    fn find_data(&self, curr: &mut usize)
    {
        while self.disc[*curr].is_none() {
            *curr -= 1;
        }
    }

    fn find_data_start(&self, curr: &mut usize)
    {
        let target = self.disc[*curr];
        while (*curr) > 0 && self.disc[*curr - 1] == target {
            *curr -= 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct MemBlock
{
    size: usize,
    _id: Option<ID>,
    start: usize,
}

impl MemBlock
{
    fn construct(size: usize, _id: Option<ID>, start: usize) -> MemBlock
    {
        MemBlock { size, _id, start }
    }
}

trait IsEven
{
    fn is_even(&self) -> bool;
}

impl IsEven for usize
{
    fn is_even(&self) -> bool {
        *self % 2 == 0
    }
}



#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn testing_data_find_end()
    {
        let mut disc = Vec::new();
        for _ in 0..10 {
            disc.push(None);
        }
        for _ in 0..10 {
            disc.push(Some(1));
        }
        let capacity = disc.len();
        let solution = Solution { disc, capacity };

        let data = solution.find_data_block(solution.capacity-1);
        println!("{:?}", data);
        assert!(data.size == 10);
    }

    #[test]
    fn testing_data_find_start()
    {
        let mut disc = Vec::new();
        for _ in 0..10 {
            disc.push(Some(1));
        }
        for _ in 0..10 {
            disc.push(None);
        }
        let capacity = disc.len();
        let solution = Solution { disc, capacity };

        let data = solution.find_data_block(solution.capacity-1);
        println!("{:?}", data);
        assert!(data.size == 10);
    }

    #[test]
    fn testing_free_find_end()
    {
        let mut disc = Vec::new();
        for _ in 0..10 {
            disc.push(None);
        }
        for _ in 0..10 {
            disc.push(Some(1));
        }
        let capacity = disc.len();
        let solution = Solution { disc, capacity };

        let data = solution.find_free_block(10);
        println!("{:?}", &data);
        assert!(data.is_some());
        assert!(data.unwrap().size == 10);
    }

    #[test]
    fn testing_free_find_start()
    {
        let mut disc = Vec::new();
        for _ in 0..10 {
            disc.push(Some(1));
        }
        for _ in 0..10 {
            disc.push(None);
        }
        let capacity = disc.len();
        let solution = Solution { disc, capacity };

        let data = solution.find_free_block(10);
        println!("{:?}", &data);
        assert!(data.is_some());
        assert!(data.unwrap().size == 10);
    }

    #[test]
    fn testing_free_find_middle()
    {
        let mut disc = Vec::new();
        for _ in 0..10 {
            disc.push(Some(1));
        }
        for _ in 0..10 {
            disc.push(None);
        }
        for _ in 0..10 {
            disc.push(Some(2));
        }
        let capacity = disc.len();
        let solution = Solution { disc, capacity };

        let data = solution.find_free_block(10);
        assert!(data.is_some());
        assert!(data.unwrap().size == 10);
        println!("{:?}", &data);
    }
}
