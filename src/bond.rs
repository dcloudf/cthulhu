#[derive(Debug, PartialEq)]
pub(crate) struct Bond {
    pub(crate) indices: (usize, usize),
    pub(crate) order: BondOrder,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum BondOrder {
    Single,
    Double,
    Triple,
    Zero, // metals
}
