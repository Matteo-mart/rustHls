use std::fs;

/// Supprime le contenu du dossier tmp_result
pub fn delete() -> std::io::Result<()> {
    let dir_to_delete = "tmp_result";
    // println!("\nSuppression du dossier de travail et de son contenu: {}", dir_to_delete);

    match fs::remove_dir_all(dir_to_delete) {
        Ok(_) => println!("Dossier supprimé avec succès."),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("Le dossier cible n'existe pas, pas de suppression nécessaire.");
        }
        Err(e) => {
            println!("Erreur lors de la suppression du dossier {}: {}", dir_to_delete, e);
        }
    }

    Ok(())
}