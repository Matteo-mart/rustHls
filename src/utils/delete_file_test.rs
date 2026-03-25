use std::fs;

/*
    Suppression du dossier "tmp_result"
*/
pub fn delete_file_test(dir_to_delete: &str) {
    
    match fs::remove_dir_all(dir_to_delete) {
        Ok(_) => println!("\n--- Suppression du dossier {} réussie ---", dir_to_delete),
        Err(e) => println!("\n--- Erreur lors de la suppression de {} : {} ---", dir_to_delete, e),
    }
}