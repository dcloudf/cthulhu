pub trait BaseGraph<N, E> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn nodes(&self) -> Vec<N>;
    fn edges(&self) -> Vec<E>;
    fn neighbours(&self, node_id: &usize) -> Vec<N>;
}
