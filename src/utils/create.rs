use std::fs;
use std::io;

pub fn create(dossier: &str) -> io::Result<()> {
    match fs::create_dir(dossier) {
        Ok(_) => Ok(()),

        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("Le dossier '{}' existe déjà.", dossier);
            Ok(())
        }

        Err(e) => {
            eprintln!("\nImpossible de créer '{}': {} ({:?})\n", dossier, e, e.kind());
            Err(e)
        }
    }
}