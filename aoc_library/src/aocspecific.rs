


use std::{fmt::{Debug, Display}, thread::sleep, time::Duration};

use crate::vector::Vec2;



pub trait CharEntity
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

pub fn empty_entity() -> char { '.' }
pub fn wall_entity() -> char { '#' }
pub fn box_left() -> char { '[' }
pub fn box_right() -> char { ']' }
pub fn robot_entity() -> char { '@' }



pub fn debugdelaydisplay<T>(message: T)
where T: Debug
{
    println!("{:?}", message);
    sleep(Duration::from_millis(50));
}

pub fn delay(timemillis: u64)
{
    sleep(Duration::from_millis(timemillis));
}

pub fn outputdisplay<T>(p1: T, t1: Duration, p2: T, t2: Duration)
where T: Display
{
    println!("\n__--__--__--__--__--__--__--__--__--__--__--__--__\n");
    println!("part one: {}\ntime one: {:?}\n", p1, t1);
    println!("part two: {}\ntime two: {:?}\n", p2, t2);
}
