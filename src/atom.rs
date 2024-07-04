use crate::element::Element;

#[derive(Debug, PartialEq)]
pub struct Atom {
    element: Element,
    charge: i8,
    hydrogens: u8,
}
