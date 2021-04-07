#![allow(dead_code)]

#[allow(unused_imports)]
mod cover_tree;
mod dsu;
mod kde;
use cover_tree::*;

use rand::prelude::*;

fn rand_point<const D: usize>(rng: &mut ThreadRng) -> [f32; D] {
    let mut arr = [0f32; D];
    for elem in arr.iter_mut() {
        *elem = rng.gen();
    }
    arr
}

fn main() {
    let mut rng = thread_rng();
    let mut cover_tree = CoverTree::new([0.0, 0.0, 0.0], 1);
    // println!("1: {:?}\n", cover_tree);

    cover_tree.insert([2.0, 2.0, 2.0]);
    // println!("2: {:?}\n", cover_tree);

    cover_tree.insert([1.0, 1.0, 1.0]);
    // println!("3: {:?}\n", cover_tree);

    cover_tree.insert([0.5, 0.5, 0.5]);
    // println!("4: {:?}\n", cover_tree);

    cover_tree.insert([1.5, 1.5, 1.5]);
    // println!("5: {:?}\n", cover_tree);

    cover_tree.insert([2.5, 2.5, 2.5]);
    // println!("6: {:?}\n", cover_tree);

    cover_tree.insert([3.5, 3.5, 3.5]);
    // println!("7: {:?}\n", cover_tree);

    cover_tree.insert([-0.5, -0.5, -0.5]);
    // println!("8: {:?}\n", cover_tree);

    cover_tree.insert([3.0, 3.0, 3.0]);
    // println!("9: {:?}\n", cover_tree);

    cover_tree.insert([-3.0, -3.0, -3.0]);
    // println!("10: {:?}\n", cover_tree);


    for _ in 0..100 {
        cover_tree.insert(rand_point(&mut rng));
    }
}
