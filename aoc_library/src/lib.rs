


pub mod vector;
pub mod uintinf;
pub mod utils;
pub mod matrix;
pub mod aocspecific;

pub type Int = i32;
pub type BigInt = i64;
pub type BigBigInt = i128;



#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn it_works()
    {
        let x: BigBigInt = 100;
        let y: i32 = 100;

        assert!(x == y as BigBigInt);
    }
}
