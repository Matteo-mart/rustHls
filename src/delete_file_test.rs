use std::fs;

/*
    Suppression du dossier "tmp_result"
*/
pub fn delete_file_test(dir_to_delete: &str) {
    
    match fs::remove_dir_all(dir_to_delete) {
        Ok(_) => println!("\nSuppression du dossier {} réussie", dir_to_delete),
        Err(e) => println!("\nErreur lors de la suppression de {} : {}", dir_to_delete, e),
    }
}