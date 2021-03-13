#![allow(dead_code)]

#[allow(unused_imports)]
mod cover_tree;
mod dsu;
mod kde;
use cover_tree::*;

fn main() {
    let mut cover_tree = CoverTree::new([0.0, 0.0, 0.0], 1);
    cover_tree.insert([2.0, 2.0, 2.0]);
    cover_tree.insert([1.0, 1.0, 1.0]);
    cover_tree.insert([0.5, 0.5, 0.5]);
}
