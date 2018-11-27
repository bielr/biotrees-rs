extern crate biotrees;

use biotrees::phylo::{Phylo, Node};
use biotrees::phylo::newick::ToNewick;

use biotrees::shape::Shape;
use biotrees::shape::generator as shape_gen;


fn sym_desc_half(t: &Shape) -> u32 {
    let (sd, _) = t.binary_fold((1u32, 1u32), |t, (sdl, kl), (sdr, kr)| {
        let kappa = kl + kr;

        match t {
            Node(ref ts) =>
                if ts[0].isomorphic(&ts[1]) {
                    (sdl * sdr * kl, kappa)
                } else {
                    (sdl * sdr, kappa)
                }
            _ => panic!()
        }
    });

    sd
}


fn sym_desc(t: &Shape) -> u32 {
    let (sd, _) = t.binary_fold((1u32, 1u32), |t, (sdl, kl), (sdr, kr)| {
        let kappa = kl + kr;

        match t {
            Node(ref ts) =>
                if ts[0].isomorphic(&ts[1]) {
                    (sdl * sdr * kappa, kappa)
                } else {
                    (sdl * sdr, kappa)
                },
            _ => panic!()
        }
    });

    sd
}


fn depth_avg<T>(t: &Phylo<T>) -> f64 {
    let depths = t.get_leaves_depths();
    let avg = depths.iter().sum::<u32>() as f64 / depths.len() as f64;
    avg
}


fn depth_var<T>(t: &Phylo<T>) -> f64 {
    let depths = t.get_leaves_depths();
    let avg = depths.iter().sum::<u32>() as f64 / depths.len() as f64;
    let var = depths.iter().map(|depth| (*depth as f64 - avg).powi(2)).sum::<f64>() / depths.len() as f64;

    var
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let binary = false;
    let nleaves = args[1].parse().unwrap();


    if binary {
        println!("newick\tcolless\tsackin\tcophenetic\tqi\tcherries\tautomorphisms");

        for t in shape_gen::all_binary_trees(nleaves).iter() {
            println!("{};\t{}\t{}\t{}\t{}\t{}\t{}",
                t.to_newick(),
                t.binary_colless_index(),
                t.sackin_index(),
                t.cophenetic_index(),
                t.quartet_index(None),
                t.count_cherries(),
                t.count_automorphisms()
            );

            assert_eq!(t.quartet_index(Some(&[0,0,0,1,1])), t.binary_quartet_index());
        }
    } else {
        println!("newick\tsackin\tcophenetic\tqi\tcherries\tautomorphisms");

        for t in shape_gen::all_trees(nleaves).iter() {
            println!("{};\t{}\t{}\t{}\t{}\t{}",
                t.to_newick(),
                t.sackin_index(),
                t.cophenetic_index(),
                t.quartet_index(None),
                t.count_cherries(),
                t.count_automorphisms()
            );
        }
    }
}
