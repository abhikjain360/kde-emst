use fnv::{FnvHashMap, FnvHashSet};

use crate::cover_tree::{covdist, distance};

pub type Point<const D: usize> = [f32; D];

/// The set to store nodes. The key is the level and value is a HashSet storing all the nodes at
/// the given level which are in the set.
pub type NodeSet = FnvHashMap<i32, FnvHashSet<i32>>;

/// Node for each point.
pub struct Node<const D: usize> {
    /// The point corresponding to the node.
    pub point: Point<D>,
    /// Children of the `Node`, stored as a HashSet.
    pub children: FnvHashSet<i32>,
    /// The index of the parent `Node`, if exists, in the level above this one.
    pub parent: Option<i32>,
    /// The maximum distance between this `Node` and any of it's descendants.
    pub max_dist: Option<i32>,
}
impl<const D: usize> Node<D> {
    pub fn new(point: Point<D>) -> Self {
        Node {
            point,
            children: FnvHashSet::default(),
            parent: None,
            max_dist: None,
        }
    }
    pub fn with_parent(point: Point<D>, parent: i32) -> Self {
        Node {
            point,
            children: FnvHashSet::default(),
            parent: Some(parent),
            max_dist: None,
        }
    }
}
