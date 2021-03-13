#![allow(dead_code)]

use crate::cover_tree::Point;
use std::sync::Arc;

// for distance
use cblas::{saxpy, snrm2};

// for quick access to all points in a given level in the CoverTree
use fnv::{FnvHashMap, FnvHashSet};

pub fn distance<const D: usize>(a: &[f32; D], b: &[f32; D]) -> f32 {
    let d = D as i32;
    let mut temp = *a;
    unsafe {
        saxpy(d, -1.0, b, 1, &mut temp, 1);
        snrm2(d, &temp, 1)
    }
}

pub fn covdist(level: i32) -> f32 {
    (2 << level as usize) as f32
}

pub fn sepdist(level: i32) -> f32 {
    (2 << (level - 1) as usize) as f32
}

pub struct Node<const D: usize> {
    pub point: [f32; D],
    // INVARIANT: keeps the index of childrens as per the next level in the the hashmap of the CoverTree, and the index
    // it points to must not be None
    pub childs: FnvHashSet<usize>,
    // is the index of parent in vector of hashmap of previous level
    pub parent: Option<usize>,
    pub max_dist: Option<i32>,
}

pub struct CoverTree<const D: usize> {
    // INVARIANT: change the values in corresponding parent nodes is indexing of things change
    // here. Also, bottom_level & top_level is never empty, though they can be the same.
    pub levels: FnvHashMap<i32, Vec<Node<D>>>,
    // INVARIANT: top_level and bottom_level must always point to valid stuff if not std::
    pub top_level: i32,
    pub bottom_level: i32,
    // root   = level[top_level][0]
    // leaves = level[bottom_level]
    pub size: usize,
}

pub struct ACoverTree<const D: usize> {
    // INVARIANT: change the values in corresponding parent nodes is indexing of things change
    // here. Also, bottom_level & top_level is never empty, though they can be the same.
    pub levels: FnvHashMap<i32, Vec<Arc<Node<D>>>>,
    // INVARIANT: top_level and bottom_level must always point to valid stuff if not std::
    pub top_level: i32,
    pub bottom_level: i32,
    // root   = level[top_level][0]
    // leaves = level[bottom_level]
    pub size: usize,
}

impl<const D: usize> Node<D> {
    pub fn new(point: [f32; D]) -> Self {
        Node {
            point,
            childs: FnvHashSet::default(),
            parent: None,
            max_dist: None,
        }
    }
}

impl<const D: usize> CoverTree<D> {
    pub fn distance(&self, point: &Point<D>) -> f32 {
        distance(point, &self.levels[&self.top_level][&0].point)
    }

    pub fn insert(&mut self, point: [f32; D]) -> Option<()> {
        while distance(&self.levels.get(&self.top_level)?[0].point, &point)
            > 2.0 * covdist(self.top_level)
        {
            self.move_leaf_to_root();
        }
        Some(())
    }
    // This should return a read lock or maybe just clone as needs to be safe to run in parallel
    pub fn nearest_neighbour(&self, point: [f32; D]) -> &[f32; D] {
        todo!()
    }

    // NOTE: this function breaks the `max_dist` invariant of the node, will need to fix
    pub fn merge(&mut self, mut other: CoverTree<D>) {
        if other.top_level > self.top_level {
            std::mem::swap(self, &mut other);
        }
        while other.top_level < self.top_level {
            other.move_leaf_to_root();
        }

        todo!()
    }

    fn merge_helper(&mut self, mut other: CoverTree<D>) -> Option<()> {
        if distance(
            &self.levels.get(&self.top_level)?[0].point,
            &other.levels.get(&self.top_level)?[0].point,
        ) < covdist(self.top_level)
        {
            return None;
        }

        enum Sets {
            Uncovered,
            SepCov,
            Leftovers,
        }

        todo!()
    }

    // NOTE: this function breaks the `max_dist` invariant of the node, will need to fix
    fn move_leaf_to_root(&mut self) -> Option<()> {
        // removing the leaf
        let leaf = self.levels.get_mut(&self.bottom_level)?.pop()?;
        // removing record from parent
        let parent = leaf.parent?;
        self.levels.get_mut(&(self.bottom_level - 1))?[parent]
            .childs
            .remove(&0);
        // inserting it to the top
        self.levels.insert(self.top_level + 1, {
            let mut v = Vec::new();
            v.push(leaf);
            v
        });
        Some(())
    }
}

impl<const D: usize> From<Vec<[f32; D]>> for CoverTree<D> {
    // needs to run in parallel
    fn from(data: Vec<[f32; D]>) -> Self {
        todo!()
    }
}
