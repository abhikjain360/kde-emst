use fnv::FnvHashMap;

pub mod dist;
pub mod cluster;
pub mod node;

pub use cluster::*;
pub use node::*;

/// A collection of cluster of nodes forms a level.
pub type Level<'p, const D: usize, const B: usize> = FnvHashMap<u8, Cluster<'p, D, B>>;

/// The cover tree.
pub struct CoverTree<'p, const D: usize, const B: usize> {
    /// The root node.
    root: Node<'p, D>,

    /// Immediate children of the root node.
    children: FnvHashMap<u8, Node<'p, D>>,

    /// All the levels in the cover tree.
    levels: FnvHashMap<i32, Level<'p, D, B>>,

    /// The bottom level, to avoid traversing the tree.
    leaf_level: i32,

    /// The root level, to avoid getting max key in `levels`.
    root_level: i32,

    /// Keeps info about the level, cluster number and the index in that cluster for each value.
    lookup: FnvHashMap<&'p Point<D>, (i32, u8, u8)>,
}
