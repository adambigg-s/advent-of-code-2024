


use std::ops::{AddAssign, MulAssign, Neg};



#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Hash, Eq)]
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

impl<T> AddAssign for Vec2<T>
where T: AddAssign + Clone + Copy
{
    fn add_assign(&mut self, other: Vec2<T>)
    {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> MulAssign for Vec2<T>
where T: MulAssign + Clone + Copy
{
    fn mul_assign(&mut self, other: Vec2<T>)
    {
        self.x *= other.x;
        self.y *= other.y;
    }
}



#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn add_test()
    {
        let mut x: Vec2<isize> = Vec2::cons(100, 100);
        let y: Vec2<isize> = Vec2::cons(-100, -200);
        x += y;

        assert!(x == Vec2::cons(0, -100));
    }
}
