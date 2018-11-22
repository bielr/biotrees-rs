use std::fmt;
use phylo::{Phylo, Leaf, Node};


struct AsNewick<'a, T> {
    t: &'a T
}

impl<'a, T: ToNewick> fmt::Display for AsNewick<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.t.fmt_newick(f)
    }
}

pub trait ToNewickLeaf {
    fn fmt_newick_leaf_str(&self, &mut fmt::Formatter) -> fmt::Result;
}

pub trait ToNewick: Sized {
    fn fmt_newick(&self, &mut fmt::Formatter) -> fmt::Result;

    fn to_newick(&self) -> String {

        let s: &Self = self;
        AsNewick { t: s }.to_string()
    }
}


impl ToNewickLeaf for u32 {
    fn fmt_newick_leaf_str(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ToNewickLeaf for i32 {
    fn fmt_newick_leaf_str(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ToNewickLeaf for str {
    fn fmt_newick_leaf_str(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T: ToNewickLeaf> ToNewick for Phylo<T> {
    fn fmt_newick(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Leaf(ref x)  => x.fmt_newick_leaf_str(f),

            Node(ref ts) => {
                try!(f.write_str("("));
                try!(ts[0].fmt_newick(f));

                for t in ts[1..].iter() {
                    try!(f.write_str(","));
                    try!(t.fmt_newick(f))
                }

                f.write_str(")")
            }
        }
    }

}

