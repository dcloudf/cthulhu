mod atom;
mod bond;
mod molecule;

use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

use molecule::Molecule;

fn read_from_xyz(path: &Path) -> Result<Molecule, Error> {
    match File::open(path) {
        Ok(mut file) => {
            let mut s = String::new();
            let _ = file.read_to_string(&mut s);
            Ok(Molecule::from(s))
        }
        Err(why) => Err(why),
    }
}

fn write_to_xyz(path: &Path, mol: &Molecule) -> Result<(), Error> {
    match File::create(path) {
        Err(why) => Err(why),
        Ok(mut file) => match file.write_all(mol.to_string().as_bytes()) {
            Ok(_) => Ok(()),
            Err(why) => Err(why),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_write_and_read_molecule() {
        let p = Path::new("mol.xyz");
        let m = Molecule::from("2\n\n  N  1.0  4.5  -3.5  \n  N  2.0  -0.5  2.5  ".to_string());
        let _ = write_to_xyz(p, &m);
        assert_eq!(read_from_xyz(p).unwrap(), m);
        let _ = remove_file(p);
    }
}
