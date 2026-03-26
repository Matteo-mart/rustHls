use std::fs;

///Création du dossier "tmp_result"
pub fn create() -> std::io::Result<()>{
    
    fs::create_dir("tmp_result")?;
    // println!("\nDossier 'tmp_result' crée");
    Ok(())

}