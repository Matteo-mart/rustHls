use std::fs;
use std::io;

pub fn create(file_tmp_result: &str) -> io::Result<()> {
    match fs::create_dir(file_tmp_result) {
        Ok(_) => Ok(()),

        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("Le dossier '{}' existe déjà.", file_tmp_result);
            Ok(())
        }

        Err(e) => {
            eprintln!("\nImpossible de créer '{}': {} ({:?})\n", file_tmp_result, e, e.kind());
            Err(e)
        }
    }
}