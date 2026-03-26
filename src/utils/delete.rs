use std::fs;
use std::io;
use std::path::Path;

pub fn delete<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    match fs::remove_dir_all(path) {
        Ok(_) => {
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("Le dossier {:?} n'existe pas, aucune action requise.", path);
            Ok(())
        }
        Err(e) => {
            eprintln!("Erreur lors de la suppression du dossier {:?}: {}", path, e);
            Err(e)
        }
    }
}