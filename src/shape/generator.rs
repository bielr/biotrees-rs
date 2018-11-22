use shape::{Shape, Leaf, Node};
use std::collections::BTreeSet;
use std::iter;
use std::sync::Arc;
use rayon::prelude::*;

use cached::cached;


pub fn iter_insert_tree<'r>(ts: &'r [Shape], t2: &'r Shape) -> impl Iterator<Item=&'r Shape> {
    let (before, after) = match ts.iter().enumerate().find(|i_t| i_t.1 >= t2) {
        None => (ts.iter(), [].iter()),
        Some((i, _)) => (ts[..i].iter(), ts[i..].iter()),
    };

    before.chain(iter::once(t2)).chain(after)
}

pub fn replace_tree_at(ts: &[Shape], n: usize, t2: Shape) -> Vec<Shape> {
    let mut result = Vec::with_capacity(ts.len());

    match ts.iter().enumerate().find(|i_t| i_t.1 >= &t2).map(|i_t| i_t.0) {

        None => {
            result.extend_from_slice(&ts[..n]);
            result.extend_from_slice(&ts[n+1..]);
            result.push(t2);
        },

        Some(i) if i == n => {
            result.extend_from_slice(&ts[..n]);
            result.push(t2);
            result.extend_from_slice(&ts[n+1..]);
        },

        Some(i) if i < n => {
            result.extend_from_slice(&ts[..i]);
            result.push(t2);
            result.extend_from_slice(&ts[i..n]);
            result.extend_from_slice(&ts[n+1..]);
        },

        Some(i) => {
            result.extend_from_slice(&ts[..n]);
            result.extend_from_slice(&ts[n+1..i]);
            result.push(t2);
            result.extend_from_slice(&ts[i..]);
        }
    };

    result
}


pub fn add_leaf_to_edge(t: Shape) -> Shape {
    Shape::shared_node(Arc::new([Shape::leaf(), t]))
}

pub fn add_leaf_to_node(t: Shape) -> Shape {
    match t {
        Leaf(..) => add_leaf_to_edge(t),
        Node(ts) => {
            let mut ts2 = Vec::with_capacity(ts.len()+1);
            ts2.push(Shape::leaf());
            ts2.extend_from_slice(&ts);
            Shape::node(ts2)
        }
    }
}


cached! {
    ALL_BINARY_TREES;

    fn all_binary_trees(n: u32) -> Arc<BTreeSet<Shape>> = {

        fn generate_from(t: Shape) -> BTreeSet<Shape> {
            let mut r = match t {
                Leaf(..) => BTreeSet::new(),

                Node(ref ts) =>
                    ts.iter().enumerate()
                        .flat_map(|(i, ch)|
                            generate_from(ch.clone()).into_iter()
                                .map(move |ch2| Shape::node(replace_tree_at(ts, i, ch2))))
                        .collect()
            };

            r.insert(add_leaf_to_edge(t));

            r
        }

        fn generate(k: u32) -> BTreeSet<Shape> {
            match k {
                0 => BTreeSet::new(),
                1 => iter::once(Shape::leaf()).collect(),
                _ =>
                    all_binary_trees(k-1).par_iter()
                        .flat_map(|t| generate_from(t.clone()))
                        .collect()
            }
        }

        Arc::new(generate(n))
    }
}

cached! {
    ALL_TREES;

    fn all_trees(n: u32) -> Arc<BTreeSet<Shape>> = {

        fn generate_from(t: Shape) -> BTreeSet<Shape> {
            let mut r = match t {
                Leaf(..) => BTreeSet::new(),

                Node(ref ts) =>
                    ts.iter().enumerate()
                        .flat_map(|(i, ch)|
                            generate_from(ch.clone()).into_iter()
                                .map(move |ch2| Shape::node(replace_tree_at(ts, i, ch2))))
                        .chain(iter::once(add_leaf_to_node(t.clone())))
                        .collect()
            };

            r.insert(add_leaf_to_edge(t));

            r
        }

        fn generate(k: u32) -> BTreeSet<Shape> {
            match k {
                0 => BTreeSet::new(),
                1 => iter::once(Shape::leaf()).collect(),
                _ =>
                    all_trees(k-1).par_iter()
                        .flat_map(|t| generate_from(t.clone()))
                        .collect()
            }
        }

        Arc::new(generate(n))
    }
}
