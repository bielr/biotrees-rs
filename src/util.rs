use std::ops::*;
use std::iter::Product;


pub fn binom2<T>(n: T) -> T
    where T: Copy + Mul<Output=T> + Sub<Output=T> + Div<Output=T> + From<u32> {

    n*(n - 1_u32.into()) / 2_u32.into()
}

pub fn factorial<T>(n: T) -> T
    where T: Copy + Add<Output=T> + Mul<Output=T> + Product + From<u32>,
          Range<T>: Iterator<Item=T> {

    let one = 1_u32.into();
    (one .. n+one).product::<T>()
}
