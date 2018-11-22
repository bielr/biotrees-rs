#[macro_use]
extern crate biotrees;

use biotrees::phylo::Phylo;
use biotrees::phylo::newick::ToNewick;
use biotrees::shape::generator as shape_gen;


fn main() {
    let t: Phylo<u32> = make_phylo!{ (1, (3), (4, 6, 8)) };

    println!("phylo {} with leaves {:?}", t.to_newick(), t.get_leaves());

    println!("phylo's shape is {}", t.clone_shape().to_newick());

    println!("phylo's leaves' depths are {:?}", t.get_leaves_depths());

    let sym: Phylo<u32> = make_phylo!{ ((1, 2), (3, 4), (5, 6)) };

    println!("aut({}) = {}", sym.to_newick(), sym.count_automorphisms());

    println!("trees with 4 leaves:");
    for t in shape_gen::all_trees(4).iter() {
        println!("\t{}", t.to_newick());
    }

    println!("# binary trees with 15 leaves: {}", shape_gen::all_binary_trees(15).len());
}
