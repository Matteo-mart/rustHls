use std::fs;
use std::io;
use std::path::Path;

/// supprime le dossier du résultat
pub fn delete<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    match fs::remove_dir_all(path) {
        Ok(_) => {
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            eprintln!("rien à supprimer");
            Ok(())
        }
        Err(e) => {
            eprintln!("\nImpossible de supprimer {:?}: {} ({:?})\n", path, e, e.kind());
            Err(e)
        }
    }
}