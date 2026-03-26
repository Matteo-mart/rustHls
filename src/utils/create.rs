use std::fs;

///Création du dossier "tmp_result"
pub fn create(file_tmp_result: &str) -> std::io::Result<()>{
    
    fs::create_dir(file_tmp_result)?;
    // println!("\nDossier 'tmp_result' crée");
    Ok(())

}