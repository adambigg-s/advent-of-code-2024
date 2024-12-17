use std::ops::Neg;




pub type Int = i32;

pub trait CharEntity
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

pub fn start() -> char { 'S' }
pub fn end() -> char { 'E' }
pub fn wall() -> char { '#' }
pub fn empty() -> char { '.' }

pub const DIRECTIONS: [Vec2<isize>; 4] = [
    Vec2::cons(0, 1), Vec2::cons(0, -1),
    Vec2::cons(1, 0), Vec2::cons(-1, 0),
];

pub fn get_directions() -> [Vec2<isize>; 4]
{
    DIRECTIONS
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Vec2<T>
{
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
{
    pub const fn cons(x: T, y: T) -> Vec2<T>
    {
        Vec2 { x, y }
    }

    pub fn rotate_cw(self) -> Vec2<T>
    where T: Neg<Output = T>
    {
        Vec2::cons(self.y, -self.x)
    }

    pub fn rotate_ccw(self) -> Vec2<T>
    where T: Neg<Output = T>
    {
        Vec2::cons(-self.y, self.x)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct State
{
    pub pos: Vec2<usize>,
    pub vel: Vec2<isize>,
    pub score: Int,
}

impl State
{
    pub fn cons(pos: Vec2<usize>, vel: Vec2<isize>, score: Int) -> State
    {
        State { pos, vel, score }
    }
}
