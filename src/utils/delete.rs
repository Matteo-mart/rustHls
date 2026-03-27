use std::fs;
use std::io;
use std::path::Path;

///Supprime le dossier "tmp_result"
pub fn delete<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    match fs::remove_dir_all(path) {
        Ok(_) => {
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("\nLe dossier {:?} n'existe pas\n", path);
            Ok(())
        }
        Err(e) => {
            eprintln!("\nErreur lors de la suppression du dossier {:?}: {}\n", path, e);
            Err(e)
        }
    }
}