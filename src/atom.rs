use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
    pub(crate) element: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl From<String> for Atom {
    fn from(value: String) -> Self {
        let mut splitted = value.split_whitespace();
        let element = splitted.next().unwrap().to_string();
        let xyz = splitted
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();
        Atom {
            element,
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  {0}    {1:.2}    {2:.2}    {3:.2}",
            self.element, self.x, self.y, self.z
        )
    }
}

impl Atom {
    pub(crate) fn calculate_pairwise_distance(self, other_atom: &Atom) -> f32 {
        ((self.x - other_atom.x).powi(2)
            + (self.y - other_atom.y).powi(2)
            + (self.z - other_atom.z).powi(2))
        .sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_pairwise_distance() {
        let a1 = Atom {
            element: "C".to_string(),
            x: 1.0,
            y: 0.0,
            z: -1.0,
        };
        let a2 = Atom {
            element: "C".to_string(),
            x: -7.0,
            y: 7.0,
            z: 1.5,
        };
        assert_eq!(a1.clone().calculate_pairwise_distance(&a2), 10.920165);
        assert_eq!(a1.clone().calculate_pairwise_distance(&a1), 0.0);
    }

    #[test]
    fn test_atom_from_string() {
        let s = "  N  1.0  4.5  -3.5  ".to_string();
        assert_eq!(
            Atom::from(s),
            Atom {
                element: "N".to_string(),
                x: 1.0,
                y: 4.5,
                z: -3.5
            }
        )
    }

    #[test]
    fn test_atom_to_string() {
        let atom = Atom {
            element: String::from("S"),
            x: 6.32,
            y: -0.55,
            z: 1.0,
        };
        assert_eq!(
            atom.to_string(),
            String::from("  S    6.32    -0.55    1.00")
        )
    }
}
