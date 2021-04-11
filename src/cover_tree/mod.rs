#![allow(dead_code)]

use std::{
    borrow::Borrow,
    collections::HashSet,
    fmt::{self, Debug, Formatter},
};

use fnv::{FnvHashMap, FnvHashSet};

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

impl<const D: usize> Debug for CoverTree<D> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("{\n")?;
        writeln!(formatter, "\tsize: {}", self.size)?;
        writeln!(formatter, "\troot_level: {:?}", self.root_level)?;
        writeln!(formatter, "\tbottom_level: {:?}", self.bottom_level)?;
        formatter.write_str("\tlevels {\n")?;
        for (level, hashmap) in &self.levels {
            writeln!(formatter, "\t\t{} :", level)?;
            for (idx, node) in hashmap {
                write!(formatter, "\t\t\t{} : {:?};\tchildren: ", idx, node.point)?;
                for child in &node.children {
                    write!(formatter, "{}, ", child)?;
                }
                formatter.write_str("\n")?;
            }
            formatter.write_str("\n")?;
        }
        formatter.write_str("\t}\n}")
    }
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
            // while the distance between point and root is greater than the covering distance of
            // the root, keep on adding new root on top, increasing the covering distance
            while point_dist > cover_dist {
                self.move_leaf_to_root();
                point_dist = self.distance(&point);
                cover_dist = covdist(self.root_level.0);
            }

            // as the point is just covered by root, and it is unlikely that any other node will
            // cover the point, insert as a child of the root.
            if self.size == 1 {
                // size == 1 means that there are no other children, so we should create a new level
                self.levels.insert(self.root_level.0 - 1, {
                    let mut h = FnvHashMap::default();
                    h.insert(0, Node::with_parent(point, self.root_level.1));
                    h
                });
                // updating root to recognize new child
                self.get_mut(self.root_level.0, self.root_level.1)?
                    .children
                    .insert(0);
                self.bottom_level = (self.root_level.0 - 1, 0);
            } else {
                // but if size > 1, then it means that there will atleast be 1 child of root, and
                // thus the level below root must exist.
                let level = self.levels.get_mut(&(self.root_level.0 - 1))?;
                let key = level.keys().max()? + 1;
                level.insert(key, Node::with_parent(point, self.root_level.1));
                // updating root to recognize new child
                self.get_mut(self.root_level.0, self.root_level.1)?
                    .children
                    .insert(key);
            }
        } else {
            self.insert_helper(self.root_level, point);
        }
        self.size += 1;
        Some(())
    }

    // TODO: make me general
    fn insert_helper(&mut self, start_node: (i32, i32), point: Point<D>) -> Option<()> {
        let mut q_level = start_node.0 - 1;
        let mut cur_node = start_node;
        let mut children = &self.get(cur_node.0, cur_node.1).children;

        loop {
            let mut descended = false;
            for q in children {
                if distance(&self.get(q_level, *q).point, &point) <= covdist(q_level) {
                    cur_node = (q_level, *q);
                    q_level -= 1;
                    descended = true;
                    break;
                }
            }
            children = &self.get(cur_node.0, cur_node.1).children;
            if !descended || children.is_empty() {
                break;
            }
        }

        match self.levels.get_mut(&q_level) {
            Some(hashmap) => {
                let key = *hashmap.keys().max().unwrap_or(&0) + 1;
                hashmap.insert(key, Node::with_parent(point, cur_node.1));
                self.get_mut(cur_node.0, cur_node.1)?.children.insert(key);
            }
            None => {
                self.levels.insert(q_level, {
                    let mut hashmap = FnvHashMap::default();
                    hashmap.insert(0, Node::with_parent(point, cur_node.1));
                    hashmap
                });
                self.get_mut(cur_node.0, cur_node.1)?.children.insert(0);
                self.bottom_level = (q_level, 0);
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
        let old_level = self.bottom_level;
        let mut leaf = self.levels.get_mut(&old_level.0)?.remove(&old_level.1)?;

        // updating self.bottom_level
        if self.levels.get(&self.bottom_level.0)?.is_empty() {
            // if no leaf nodes left ...
            let mut new_bottom_level = self.bottom_level.0 + 1;
            let mut old_bottom_level = self.bottom_level.0;
            // ... move up until we find new node ...
            loop {
                match self.levels.get(&new_bottom_level) {
                    Some(level) => {
                        if !level.is_empty() {
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

            for level in old_bottom_level..new_bottom_level {
                self.levels.remove(&level);
            }
        } else {
            // else get the next leaf on same level, and change index at self.bottom_level
            self.bottom_level.1 = *self.levels.get(&self.bottom_level.0)?.keys().min()?;
        }

        // update parent to forget about the leaf node
        self.get_mut(old_level.0 + 1, leaf.parent?)?
            .children
            .remove(&old_level.1);

        // adding current root as a child of leaf
        leaf.children.insert(self.root_level.1);

        // adding leaf as new root
        let mut hashmap = FnvHashMap::default();
        hashmap.insert(0, leaf);
        self.levels.insert(self.root_level.0 + 1, hashmap);
        self.root_level.0 += 1;
        self.root_level.1 = 0;

        Some(())
    }

    pub fn merge(&mut self, mut other: CoverTree<D>) -> Option<()> {
        let new_size = self.size + other.size;
        if self.root_level.0 < other.root_level.0 {
            std::mem::swap(self, &mut other);
        }
        while other.root_level.0 < self.root_level.0 {
            other.move_leaf_to_root();
        }
        let other_root_level = other.root_level;
        let leftovers_others = self.merge_helper(self.root_level, &mut other, other_root_level);
        for node in leftovers_others.into_iter() {
            self.insert_helper(
                self.root_level,
                other
                    .levels
                    .get_mut(&(other.root_level.0 - 1))?
                    .remove(&node)?
                    .point,
            );
        }
        self.size = new_size;
        Some(())
    }

    // Invariants:
    // - self.root_level == other.root_level
    // - distance(self, other)<= covdist(self.root_level)
    fn merge_helper(
        &mut self,
        start_node: (i32, i32),
        other: &mut CoverTree<D>,
        other_start_node: (i32, i32),
    ) -> FnvHashSet<i32> {
        let children_prime = self.get(start_node.0, start_node.1).children.clone();

        // NOTE: all these sets contains nodes only at level = other_start_node.0 - 1,
        // and this is also the level of the notes returned from function.
        let (mut uncovered, mut sepcov, mut leftovers) = (
            FnvHashSet::default(),
            FnvHashSet::default(),
            FnvHashSet::default(),
        );

        for r in other
            .get(other_start_node.0, other_start_node.1)
            .children
            .clone()
            .into_iter()
        {
            if distance(
                &self.get(start_node.0, start_node.1).point,
                &other.get(other_start_node.0, other_start_node.1).point,
            ) < covdist(start_node.0)
            {
                let mut found_match = false;
                for &s in children_prime.iter() {
                    if distance(
                        &self.get(start_node.0 - 1, s).point,
                        &other.get(other_start_node.0 - 1, r).point,
                    ) < sepdist(start_node.0)
                    {
                        let returned_set = self.merge_helper(
                            (start_node.0 - 1, s),
                            other,
                            (other_start_node.0 - 1, r),
                        );
                        leftovers.extend(returned_set);
                        found_match = true;
                        break;
                    }
                }
                if !found_match {
                    sepcov.insert(r);
                }
            } else {
                uncovered.insert(r);
            }
        }

        for new_child in sepcov {
            self.insert(other.get(other_start_node.0 - 1, new_child).point);
        }

        self.insert_helper(
            start_node,
            other.get(other_start_node.0, other_start_node.1).point,
        );

        for r in leftovers.clone().into_iter() {
            if distance(
                &self.get(start_node.0, start_node.1).point,
                &other.get(other_start_node.0 - 1, r).point,
            ) <= covdist(start_node.0)
            {
                self.insert_helper(start_node, other.get(other_start_node.0 - 1, r).point);
                leftovers.remove(&r);
            }
        }

        leftovers.extend(uncovered);
        leftovers
    }
}
