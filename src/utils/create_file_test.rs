use std::fs;

///creation du dossier "tmp_result"
pub fn create_file_test(dir_name: &str) {

    match fs::create_dir_all(dir_name) {
        Ok(_) => println!("\n--- Création du dossier {} réussie ---", dir_name),
        Err(e)=> println!("\n--- Erreur lors de la suppression de {} : {} ---", dir_name, e),
    }
}