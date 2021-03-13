#![allow(dead_code)]

use std::borrow::Borrow;

use fnv::FnvHashMap;

mod node;
use node::*;
pub use node::{Node, Point};
mod dist;
pub use dist::*;

/// The single-threaded cover tree.
pub struct CoverTree<const D: usize> {
    /// HashMap corresponding to various levels.
    pub levels: FnvHashMap<i32, FnvHashMap<i32, Node<D>>>,
    /// The highest level in the cover tree, and the index in HashMap where the root exists.
    pub root_level: (i32, i32),
    /// The lowest level in the cover tree, and the index in HashMap where first leaf exists.
    pub bottom_level: (i32, i32),
    /// Total number of nodes in the cover tree.
    pub size: usize,
}

impl<const D: usize> CoverTree<D> {
    /// Creates a new cover tree with the given point as root at the given level. Will also be the
    /// only leaf as no other point exists for now.
    pub fn new(point: Point<D>, level: i32) -> Self {
        CoverTree {
            levels: {
                // creating a hashmap with keys as levels and values as hashmap with keys as index
                // and nodes as values. I like confusing the future me.
                let mut root_level = FnvHashMap::default();
                let mut root_level_hashmap = FnvHashMap::default();
                root_level_hashmap.insert(0, Node::new(point));
                root_level.insert(level, root_level_hashmap);
                root_level
            },
            root_level: (level, 0),
            bottom_level: (level, 0),
            size: 1,
        }
    }
    /// Calculate the distance between the root and the given point.
    #[inline]
    pub fn distance(&self, point: &Point<D>) -> f32 {
        distance(point, &self.get(self.root_level.0, self.root_level.1).point)
    }

    /// Retrieve an immutable reference to a node.
    #[inline]
    pub fn get(&self, level: i32, index: i32) -> &Node<D> {
        &self.levels[&level][&index]
    }

    /// Retrieve a mutable reference to a node.
    #[inline]
    pub fn get_mut(&mut self, level: i32, index: i32) -> Option<&mut Node<D>> {
        self.levels.get_mut(&level)?.get_mut(&index)
    }

    /// Insert an point into the cover tree.
    pub fn insert(&mut self, point: Point<D>) -> Option<()> {
        let mut point_dist = self.distance(&point);
        let mut cover_dist = covdist(self.root_level.0);
        if point_dist > cover_dist {
            while point_dist > cover_dist {
                self.move_leaf_to_root();
                point_dist = self.distance(&point);
                cover_dist = covdist(self.root_level.0);
            }
            if self.size == 1 {
                self.levels.insert(self.root_level.0 - 1, {
                    let mut h = FnvHashMap::default();
                    h.insert(0, Node::with_parent(point, self.root_level.1));
                    h
                });
                self.get_mut(self.root_level.0, self.root_level.1)?
                    .children
                    .insert(0);
                self.bottom_level = (self.root_level.0 - 1, 0);
            } else {
                let level = self.levels.get_mut(&(self.root_level.0 - 1))?;
                let key = level.keys().max()? + 1;
                level.insert(key, Node::with_parent(point, self.root_level.1));
                self.get_mut(self.root_level.0, self.root_level.1)?
                    .children
                    .insert(key);
            }
        } else {
            self.insert_helper(point, self.root_level);
        }
        self.size += 1;
        Some(())
    }

    fn insert_helper(&mut self, point: Point<D>, start_node: (i32, i32)) -> Option<()> {
        let q_level = start_node.0 - 1;
        // a level at start_level.0 is guaranteed to exist, as only valid arguments are provided
        for q in self.get(start_node.0, start_node.1).children.clone() {
            if distance(&self.get(q_level, q).point, &point) <= covdist(start_node.0 - 1) {
                self.insert_helper(point, (q_level, q));
                return Some(());
            }
        }
        match self.levels.get_mut(&q_level) {
            Some(hashmap) => {
                let key = *hashmap.keys().max()? + 1;
                hashmap.insert(key, Node::with_parent(point, start_node.1));
                self.get_mut(start_node.0, start_node.1)?
                    .children
                    .insert(key);
            }
            None => {
                self.levels.insert(q_level, {
                    let mut hashmap = FnvHashMap::default();
                    hashmap.insert(0, Node::with_parent(point, start_node.1));
                    hashmap
                });
                self.get_mut(start_node.0, start_node.1)?.children.insert(0);
            }
        }

        Some(())
    }

    /// Move a leaf node to the root, increasing the root level by 1. Make the current root the
    /// only child of the new root.
    pub fn move_leaf_to_root(&mut self) -> Option<()> {
        // 1 node, is the root and leaf at same time. But this function was called to increase
        // the root_level, so that's what we are going to do.
        if self.size == 1 {
            // get a leaf node
            let leaf = self
                .levels
                .remove(&self.root_level.0)?
                .remove(&self.root_level.1)?;
            // putting it on a level above
            self.levels.insert(self.root_level.0 + 1, {
                let mut hashmap = FnvHashMap::default();
                hashmap.insert(0, leaf);
                hashmap
            });
            // updating corresponding stuff
            self.root_level = (self.root_level.0 + 1, 0);
            self.bottom_level = self.root_level;

            return Some(());
        }

        // get a leaf node
        let mut leaf = self
            .levels
            .get_mut(&self.bottom_level.0)?
            .remove(&self.bottom_level.1)?;

        // updating self.bottom_level
        if self.levels.get(&self.bottom_level.0)?.len() == 0 {
            // if no leaf nodes left ...
            let mut new_bottom_level = self.bottom_level.0 + 1;
            // ... move up until we find new node ...
            loop {
                match self.levels.get(&new_bottom_level) {
                    Some(level) => {
                        if level.len() > 0 {
                            // ... and set that as new bottom
                            self.bottom_level = (new_bottom_level, *level.keys().min()?);
                            break;
                        } else {
                            // empty levels need not exist
                            self.levels.remove(&new_bottom_level);
                            new_bottom_level += 1;
                        }
                    }
                    None => {
                        new_bottom_level += 1;
                    }
                }
            }
        } else {
            // else get the next leaf on same level, and change index at self.bottom_level
            self.bottom_level.1 = *self.levels.get(&self.bottom_level.0)?.keys().min()?;
        }

        // update parent to forget about the leaf node
        let leaf_index = self.bottom_level.1;
        self.get_mut(self.bottom_level.0 + 1, leaf.parent?)?
            .children
            .remove(&leaf_index);

        // adding current root as a child of leaf
        leaf.children.insert(self.root_level.1);

        // adding leaf as new root
        self.levels.insert(self.root_level.0 + 1, {
            let mut hashmap = FnvHashMap::default();
            hashmap.insert(0, leaf)?;
            hashmap
        })?;
        self.root_level.0 += 1;
        self.root_level.1 = 0;

        Some(())
    }

    fn children_as_set(&self, level: i32, index: i32) -> NodeSet {
        let mut set = FnvHashMap::default();
        set.insert(level - 1, self.get(level, index).children.clone());
        set
    }
}
