#![allow(dead_code)]

use std::thread;

use rand::prelude::*;

mod cover_tree;
mod dsu;
mod kde;
use cover_tree::*;

fn rand_point<const D: usize>(rng: &mut ThreadRng) -> [f32; D] {
    let mut arr = [0f32; D];
    for elem in arr.iter_mut() {
        *elem = rng.gen::<i32>() as f32;
    }
    arr
}

fn main() {
    // let mut rng = thread_rng();
    let j1 = thread::spawn(|| {
        let mut ct1 = CoverTree::new([0.0, 0.0, 0.0], 1);
        ct1.insert([2.0, 2.0, 2.0]);
        ct1.insert([1.0, 1.0, 1.0]);
        ct1.insert([0.5, 0.5, 0.5]);
        ct1.insert([1.5, 1.5, 1.5]);
        ct1.insert([2.5, 2.5, 2.5]);
        ct1.insert([3.5, 3.5, 3.5]);
        ct1.insert([-0.5, -0.5, -0.5]);
        ct1.insert([3.0, 3.0, 3.0]);
        ct1.insert([-3.0, -3.0, -3.0]);
        ct1
    });

    let j2 = thread::spawn(|| {
        let mut ct2 = CoverTree::new([4.0, 4.0, 4.0], 1);
        ct2.insert([4.5, 4.5, 4.5]);
        ct2.insert([5.5, 5.5, 5.5]);
        ct2.insert([3.0, 3.0, 3.0]);
        ct2.insert([5.0, 5.0, 5.0]);
        ct2
    });

    let mut ct1 = j1.join().unwrap();
    let ct2 = j2.join().unwrap();

    println!("{:?}\n\n{:?}\n\n", ct1, ct2);

    ct1.merge(ct2);

    println!("{:?}\n", ct1);
}
