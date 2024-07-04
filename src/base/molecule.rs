use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct GraphError {}

impl Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Graph-based error is happened")
    }
}

impl Error for GraphError {}

#[allow(dead_code)]
pub trait BaseMolecule<E> {
    fn element(&self, id: &usize) -> Result<E, GraphError>;

    fn hydrogens(&self, id: &usize) -> Result<u8, GraphError>;

    fn charge(&self, id: &usize) -> Result<i8, GraphError>;
}
