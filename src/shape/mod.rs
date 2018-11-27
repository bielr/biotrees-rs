use std::fmt;
use lazy_static::lazy_static;

pub use phylo::{Phylo, Leaf, Node};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tip {}

pub type Shape = Phylo<Tip>;

#[macro_export]
macro_rules! make_shape {
    ( * ) => {
        $crate::shape::Shape::leaf()
    };

    ( $x:ident ) => {
        $x
    };

    ( ( $( $children:tt ),+ ) ) => {
        $crate::shape::Shape::shared_node(
            std::sync::Arc::new([ $(make_shape!($children)),* ]))
    };
}

lazy_static! {
    static ref CHERRY: Shape = make_shape!( (*, *) );
}

impl fmt::Debug for Tip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*")
    }
}

impl Shape {
    pub fn leaf() -> Self {
        Leaf(Tip{})
    }

    pub fn cherry() -> Shape {
        make_shape!{ (*, *) }
    }
}

impl<T> Phylo<T> {
    pub fn clone_shape(&self) -> Shape {
        match self {
            Leaf(..)     => Shape::leaf(),
            Node(ref ts) => Shape::node(ts.iter().map(|ch| ch.clone_shape()).collect())
        }
    }
}

pub mod balance;
pub mod generator;
pub mod newick;
