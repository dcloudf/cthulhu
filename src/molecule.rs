use std::{error::Error, fmt::Display};

use crate::{atom::Atom, bond::Bond, element::Element};

#[derive(Debug)]
pub struct GraphError {}

impl Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Graph-based error is happened")
    }
}

impl Error for GraphError {}

pub trait BaseMolecule {
    fn element(&self, id: &usize) -> Result<Element, GraphError>;

    fn hydrogens(&self, id: &usize) -> Result<u8, GraphError>;

    fn charge(&self, id: &usize) -> Result<i8, GraphError>;
}

pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
