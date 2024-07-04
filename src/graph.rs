pub trait Graph<N, E> {
    type NodeIterator: Iterator<Item = N>;
    type NeighbourIterator: Iterator<Item = N>;
    type EdgeIterator: Iterator<Item = E>;

    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn nodes(&self) -> Self::NodeIterator;
    fn edges(&self) -> Self::EdgeIterator;
    fn neighbours(&self) -> Self::NeighbourIterator;
}
