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

macro_rules! map_indices {
    ( $var:ident in $range:expr, $($var_n:ident in $range_n:expr),+ => $body:expr ) => {
        $range.flat_map(move |$var|
            map_indices!( $($var_n in $range_n),* => $body ))
    };

    ( $var:ident in $range:expr => $body:expr ) => {
        $range.map(move |$var| $body)
    };
}
