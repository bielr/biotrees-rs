pub use phylo::{Phylo, Leaf, Node};

pub type Shape = Phylo<()>;

#[macro_export]
macro_rules! make_shape {
    ( * ) => {
        $crate::shape::Shape::leaf()
    };
    ( ( $( $children:tt ),+ ) ) => {
        $crate::shape::Shape::shared_node(
            std::rc::Rc::new([ $(make_shape!($children)),* ]))
    };
}

impl Shape {
    pub fn leaf() -> Self {
        Leaf(())
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
