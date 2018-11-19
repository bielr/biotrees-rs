use phylo::{Phylo, Leaf, Node};
use std::collections::BTreeSet;
use std::rc::Rc;

static mut bin_gen_cache: Vec<BTreeSet<Phylo<u32>>> = vec![];


pub fn all_binary_trees(n: u32) -> &'static BTreeSet<Phylo<u32>> {
    fn generate_from(t: &Phylo<u32>) -> Vec<Phylo<u32>> {
        let r = match t {
            Leaf(..) => BTreeSet::new(),

            Node(ref ts) =>
                ts.iter().enumerate()
                    .flat_map(|(i, ch)|
                        generate_from(ch).iter()
                            .map(|ch2| {
                                let mut ts2 = ts.clone();
                                Rc::make_mut(&mut ts2)[i] = ch2;
                                Phylo::shared_node(ts2);
                            }))
                    .collect()
        };

        r
    }

    fn generate(k: u32) -> BTreeSet<Phylo<u32>> {
        match k {
            0 => vec![],
            _ =>
                all_binary_trees(k-1).iter()
                    .flat_map(generate_from)
                    .collect()
        }
    }

    if bin_gen_cache.len() < n as usize {
        bin_gen_cache.resize(n as usize, BTreeSet::new());
    }

    if bin_gen_cache[n as usize].is_empty() {
        bin_gen_cache[n as usize] = generate(n);
    }

    &bin_gen_cache[n as usize]
}
