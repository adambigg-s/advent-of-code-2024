


use crate::Int;
use crate::vector::Vec2;



#[derive(Debug)]
pub struct MatEq2
{
    mat: Vec<Vec<Int>>,
    vec: Vec<Int>,
}

impl MatEq2
{
    pub fn cons_list(a: Vec<Int>, vec: Vec<Int>) -> MatEq2
    {
        let mat = vec![
            vec![a[0], a[1]],
            vec![a[2], a[3]],
        ];

        MatEq2 { mat, vec }
    }

    pub fn cramer(&self) -> Option<Vec2<Int>>
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

    pub fn solves(&self, x: Int, y: Int) -> bool
    {
        self.mat[0][0] * x + self.mat[0][1] * y == self.vec[0]
            && self.mat[1][0] * x + self.mat[1][1] * y == self.vec[1]
    }
}
