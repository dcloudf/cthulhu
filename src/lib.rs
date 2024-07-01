mod atom;
mod bond;
mod molecule;

use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

use molecule::Molecule;

fn read_from_xyz(path: &str) -> Result<Molecule, Error> {
    let p = Path::new(path);
    match File::open(p) {
        Ok(mut file) => {
            let mut s = String::new();
            let _ = file.read_to_string(&mut s);
            Ok(Molecule::from(s))
        }
        Err(why) => Err(why)
    }
}

fn write_to_xyz(path: &str, mol: &Molecule) -> Result<(), Error> {
    let p = Path::new(path);
    match File::create(p) {
        Err(why) => Err(why),
        Ok(mut file ) => {
            match file.write_all(mol.to_string().as_bytes()) {
                Ok(_) => Ok(()),
                Err(why) => Err(why),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_write_and_read_molecule() {
        let m = Molecule::from("2\n\n  N  1.0  4.5  -3.5  \n  N  2.0  -0.5  2.5  ".to_string());
        let _ = write_to_xyz("mol.xyz", &m);
        assert_eq!(read_from_xyz("mol.xyz").unwrap(), m);
        let _ = remove_file(Path::new("mol.xyz"));
    }
}
