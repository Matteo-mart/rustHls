use std::fs;

/*
    Affichage du contenu du fichier 'tmp_result'
*/
pub fn affichage_file_test(dir_name: &str) {
    println!("\n--- Affichage du dossier: {} ---", dir_name);

    match fs::read_dir(dir_name) {
        Err(e) => println!("\n--- Erreur lors de la lecture de {} : {}", dir_name, e),
        Ok(entries) => {
            let files: Vec<_> = entries.filter_map(|e| e.ok()).collect();

            if files.is_empty() {
                println!("\n--- Dossier vide ---");
            } else {
                println!("\n--- Fichiers dans {}:", dir_name);
                for file in files {
                    println!(" - {}", file.file_name().to_string_lossy());
                }
            }
        }
    }

}