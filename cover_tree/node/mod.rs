use nalgebra::SVector;

/// A D-dimensional point using `f32` type.
pub type Point<const D: usize> = SVector<f32, D>;

/// A Node in the tree, containing a reference to a Point<D>, and index of the parent.
pub struct Node<'p, const D: usize> {
    /// Reference to the point.
    point: &'p Point<D>,

    /// Reference to parent.
    parent: Option<&'p Point<D>>,

    /// Index of the child cluster in next level.
    children: Option<u8>,
}
