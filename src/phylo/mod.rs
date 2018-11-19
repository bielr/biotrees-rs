use std::rc::Rc;
use std::cmp::Ordering;


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Phylo<T> {
    Leaf(T),
    Node(Rc<[Phylo<T>]>)
}

pub use self::Phylo::{Leaf, Node};
pub mod balance;
pub mod generator;

#[macro_export]
macro_rules! make_phylo {
    ( ( $( $children:tt ),+ ) ) => {
        $crate::phylo::Phylo::shared_node(
            std::rc::Rc::new([ $(make_phylo!($children)),* ]))
    };
    ( $label:expr ) => {
        $crate::phylo::Phylo::leaf_with($label)
    };
}


impl<T> Phylo<T> {
    pub fn leaf_with(label: T) -> Self {
        Leaf(label)
    }

    pub fn shared_node(children: Rc<[Self]>) -> Self {
        Node(children)
    }

    pub fn node(children: Vec<Self>) -> Self {
        Self::shared_node(Rc::from(children.into_boxed_slice()))
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Leaf(..) => true,
            Node(..) => false
        }
    }

    pub fn depth(&self) -> u32 {
        match self {
            Leaf(..)     => 0,
            Node(ref ts) => ts.iter().map(|ch| ch.depth()).max().unwrap() + 1
        }
    }

    fn get_leaves_depths_plus(&self, rec_depth: u32) -> Vec<u32> {
        match self {
            Leaf(..)     => vec![rec_depth],
            Node(ref ts) => ts.iter().flat_map(|ch| ch.get_leaves_depths_plus(rec_depth+1)).collect()
        }
    }

    pub fn get_leaves_depths(&self) -> Vec<u32> {
        self.get_leaves_depths_plus(0)
    }

    pub fn get_leaves<'r>(&'r self) -> Vec<&'r T> {
        match self {
            Leaf(ref x)  => {
                [x].iter().map(|&x| x).collect()
            },
            Node(ref ts) => {
                ts.iter()
                    .flat_map(|ch| ch.get_leaves())
                    .collect()
            }
        }
    }

    pub fn isomorphic(&self, other: &Self) -> bool {
        match (self, other) {
            (Leaf(..), Leaf(..)) => true,
            (Leaf(..), Node(..)) => false,
            (Node(..), Leaf(..)) => false,

            (Node(ref ts1), Node(ref ts2)) =>
                ts1.len() == ts2.len() &&
                    ts1.iter()
                        .zip(ts2.iter())
                        .all(|(t1,t2)| t1.isomorphic(t2))
        }
    }

    pub fn cmp_shape_grlex(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Leaf(..), Leaf(..)) => Ordering::Equal,
            (Leaf(..), Node(..)) => Ordering::Less,
            (Node(..), Leaf(..)) => Ordering::Greater,

            (Node(ref ts1), Node(ref ts2)) => {
                match ts1.len().cmp(&ts2.len()) {
                    Ordering::Equal => {
                        for (t1, t2) in ts1.iter().zip(ts2.iter()) {
                            match t1.cmp_shape_grlex(t2) {
                                Ordering::Equal => {},
                                c               => { return c; }
                            }
                        }

                        return Ordering::Equal;
                    },
                    c => c
                }
            }
        }
    }

    pub fn binary_fold<R, F>(&self, leaf_value: R, mut f: F) -> R
        where F: FnMut(R, R) -> R,
              R: Copy {

        match self {
            Leaf(..)     => leaf_value,

            Node(ref ts) => {
                assert_eq!(ts.len(), 2);

                let r0 = ts[0].binary_fold(leaf_value, |t1,t2| f(t1,t2));
                let r1 = ts[1].binary_fold(leaf_value, |t1,t2| f(t1,t2));
                f(r0, r1)
            }
        }
    }


    pub fn fold<R, F>(&self, leaf_value: R, f: F) -> R
        where F: Fn(&mut Iterator<Item=R>) -> R,
              R: Copy {

        match self {
            Leaf(..)     => leaf_value,
            Node(ref ts) => {
                let mut it = ts.iter().map(|t| t.fold(leaf_value, |iter| f(iter)));
                f(&mut it)
            }
        }
    }
}

impl<T> PartialOrd for Phylo<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Phylo<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Leaf(ref x), Leaf(ref y)) => x.cmp(y),
            (Leaf(..), Node(..))       => Ordering::Less,
            (Node(..), Leaf(..))       => Ordering::Greater,

            (Node(ref ts1), Node(ref ts2)) => {
                match ts1.len().cmp(&ts2.len()) {
                    Ordering::Equal =>
                        ts1.iter().zip(ts2.iter())
                            .map(|(t1, t2)| t1.cmp(t2))
                            .filter(|&o| o != Ordering::Equal)
                            .next()
                            .unwrap_or(Ordering::Equal),
                    c => c
                }
            }
        }
    }
}
