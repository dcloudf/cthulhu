use crate::base::{
    graph::BaseGraph,
    molecule::{BaseMolecule, GraphError},
};
use crate::{atom::Atom, bond::Bond, element::Element};

#[allow(dead_code)]
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

    fn neighbours(&self, _node_id: &usize) -> Vec<Atom> {
        todo!()
    }
}

impl BaseMolecule<Element> for Molecule {
    fn charge(&self, _id: &usize) -> Result<i8, GraphError> {
        todo!()
    }

    fn element(&self, _id: &usize) -> Result<Element, GraphError> {
        todo!()
    }

    fn hydrogens(&self, _id: &usize) -> Result<u8, GraphError> {
        todo!()
    }
}
