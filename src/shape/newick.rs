use std::fmt;

pub use phylo::newick::{ToNewickLeaf, ToNewick};
pub use shape::Tip;

impl ToNewickLeaf for Tip {
    fn fmt_newick_leaf_str(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("*")
    }
}
