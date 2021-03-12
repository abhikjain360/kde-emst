#![allow(dead_code)]

use std::sync::RwLock;

// for distance
use cblas::{saxpy, snrm2};

// for quick access to all points in a given level in the CoverTree
use fnv::FnvHashMap;

pub fn distance<const D: usize>(a: &[f32; D], b: &[f32; D]) -> f32 {
    let d = D as i32;
    let mut temp = *a;
    unsafe {
        saxpy(d, -1.0, b, 1, &mut temp, 1);
        snrm2(d, &temp, 1)
    }
}

pub struct Node<const D: usize> {
    pub point: [f32; D],
    // INVARIANT: keeps the index of childrens as per the next level in the the hashmap of the CoverTree, and the index
    // it points to must not be None
    pub childs: Vec<usize>,
}

impl<const D: usize> Node<D> {
    pub fn new(point: [f32; D]) -> Self {
        Node {
            point,
            childs: Vec::new(),
        }
    }
}

pub struct CoverTree<const D: usize> {
    // INVARIANT: change the values in corresponding parent nodes is indexing of things change here
    pub levels: FnvHashMap<i32, Vec<Option<RwLock<Node<D>>>>>,
    // INVARIANT: top_level and bottom_level must always point to valid stuff if not std::
    pub top_level: i32,
    pub bottom_level: i32,
    // root   = level[top_level][0]
    // leaves = level[bottom_level]
}

impl<const D: usize> CoverTree<D> {
    pub fn insert(&mut self) {}
    // This should return a read lock or maybe just clone as needs to be safe to run in parallel
    pub fn nearest_neighbour(&self, point: [f32; D]) -> &[f32; D] {
        todo!()
    }

    pub fn merge(&mut self, mut other: CoverTree<D>) {}
    fn merge_helper(&mut self, mut other: CoverTree<D>) {}
}

impl<const D: usize> From<Vec<[f32; D]>> for CoverTree<D> {
    // needs to run in parallel
    fn from(data: Vec<[f32; D]>) -> Self {
        todo!()
    }
}
