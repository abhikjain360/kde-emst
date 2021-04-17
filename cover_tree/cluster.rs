use fnv::FnvHashMap;

use super::Node;

/// A cluster of nodes.
pub struct Cluster<'p, const D: usize, const B: usize> {
    /// Nodes in the cluster.
    nodes: FnvHashMap<u8, Node<'p, D>>,

    /// Index of the next cluster.
    next: Option<u8>,
}
