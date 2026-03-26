use std::fs;

/// affichage du dossier "tmp_result"
pub fn affichage_dossier() -> std::io::Result<()>{

    println!("\nVoici le contenu du dossier 'tmp_result': ");
    for entry in fs::read_dir("tmp_result")? {
        let entry = entry?;
        println!("{:?}", entry.file_name());
    }
    Ok(())

}