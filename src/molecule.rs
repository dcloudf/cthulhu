use crate::{atom::Atom, bond::Bond, element::Element};
use crate::base::{graph::BaseGraph, molecule::{BaseMolecule, GraphError}};

pub struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}

impl BaseGraph<Atom, Bond> for Molecule {
    fn is_empty(&self) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn nodes(&self) -> Vec<Atom> {
        todo!()
    }

    fn edges(&self) -> Vec<Bond> {
        todo!()
    }

    fn neighbours(&self, node_id: &usize) -> Vec<Atom> {
        todo!()
    }
}

impl BaseMolecule<Element> for Molecule {
    fn charge(&self, id: &usize) -> Result<i8, GraphError> {
        todo!()
    }

    fn element(&self, id: &usize) -> Result<Element, GraphError> {
        todo!()
    }

    fn hydrogens(&self, id: &usize) -> Result<u8, GraphError> {
        todo!()
    }
}
