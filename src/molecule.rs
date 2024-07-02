use crate::atom::Atom;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display};

const RADII_TOLERANCE: f32 = 0.4;

lazy_static! {
    static ref COVALENT_RADII: HashMap<String, f32> = HashMap::from([
        ("H".to_string(), 0.23),
        ("B".to_string(), 0.83),
        ("C".to_string(), 0.68),
        ("N".to_string(), 0.68),
        ("O".to_string(), 0.68),
        ("F".to_string(), 0.64),
        ("Si".to_string(), 1.20),
        ("P".to_string(), 1.05),
        ("S".to_string(), 1.02),
        ("Cl".to_string(), 0.99),
        ("As".to_string(), 1.21),
        ("Se".to_string(), 1.22),
        ("Br".to_string(), 1.21),
        ("Te".to_string(), 1.47),
        ("I".to_string(), 1.40),
    ]);
}

#[derive(Debug, PartialEq)]
pub(crate) struct Molecule {
    pub(crate) atoms: Vec<Atom>,
    pub(crate) connectivity: Vec<(usize, usize)>,
}

impl From<String> for Molecule {
    fn from(value: String) -> Self {
        Molecule {
            atoms: value
                .lines()
                .skip(2)
                .map(|x| Atom::from(x.to_string()))
                .collect::<Vec<Atom>>(),
            connectivity: Vec::new(),
        }
    }
}

impl Display for Molecule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("{}\n\n", self.atoms.len());
        for atom in self.atoms.iter() {
            s.push_str(format!("{}\n", atom).as_str())
        }
        write!(f, "{}", s)
    }
}

impl Molecule {
    fn calculate_atomic_connectivity(&mut self) {
        self.connectivity = self
            .atoms
            .iter()
            .enumerate()
            .combinations(2)
            .filter_map(|vector| {
                let (idx, x) = vector[0];
                let (idy, y) = vector[1];
                match x.clone().calculate_pairwise_distance(y)
                    < COVALENT_RADII.get(&x.element).unwrap()
                        + COVALENT_RADII.get(&y.element).unwrap()
                        + RADII_TOLERANCE
                {
                    true => Some((idx, idy)),
                    false => None,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_molecule_from_string() {
        let m: String = "2\n\n  N  1.0  4.5  -3.5  \n  N  2.0  -0.5  2.5  ".into();
        assert_eq!(
            Molecule::from(m),
            Molecule {
                atoms: vec![
                    Atom {
                        element: "N".to_string(),
                        x: 1.0,
                        y: 4.5,
                        z: -3.5
                    },
                    Atom {
                        element: "N".to_string(),
                        x: 2.0,
                        y: -0.5,
                        z: 2.5
                    }
                ],
                connectivity: Vec::new(),
            }
        )
    }

    #[test]
    fn test_calculate_atomic_connectivity() {
        let mut m = Molecule::from("8\n\nC        2.99666        0.54725       -0.11662\nC        1.85093        1.48275        0.35824\nC        0.47625        0.76558        0.61847\nC        0.30612       -0.21706        1.84994\nC        1.68372        2.63281       -0.67746\nC        0.76997       -1.67782        1.52681\nC       -1.21693       -0.30733        2.21427\nC        1.05570        0.29685        3.12128".to_string());
        m.calculate_atomic_connectivity();
        assert_eq!(
            m.connectivity,
            vec![(0, 1), (1, 2), (1, 4), (2, 3), (3, 5), (3, 6), (3, 7)]
        )
    }

    #[test]
    fn test_molecule_to_string() {
        let mol = Molecule {
            atoms: vec![
                Atom {
                    element: "N".to_string(),
                    x: 1.0,
                    y: 4.5,
                    z: -3.5,
                },
                Atom {
                    element: "N".to_string(),
                    x: 2.0,
                    y: -0.5,
                    z: 2.5,
                },
            ],
            connectivity: Vec::new(),
        };
        assert_eq!(
            mol.to_string(),
            String::from("2\n\n  N    1.00    4.50    -3.50\n  N    2.00    -0.50    2.50\n")
        )
    }
}
