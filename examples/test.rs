#[macro_use]
extern crate biotrees;

fn main() {
    let t = make_phylo!{ (1, (3), (4, 6, 8)) };

    println!("mah phylo {:?} with leaves {:?}", t, t.get_leaves());

    println!("mah phylo's shape is {:?}", t.clone_shape());

    println!("mah phylo's leaves' depths are {:?}", t.get_leaves_depths());


    let sym = make_phylo!{ ((1, 2), (3, 4), (5, 6)) };

    println!("automorphisms of {:?} = {}", sym, sym.count_automorphisms());
}
