use std::fs;

///Suppression du dossier "tmp_result"
pub fn delete() -> std::io::Result<()>{
 
    fs::remove_dir("tmp_result")?;
    println!("\nDossier 'tmp_result' supprimés");
    Ok(())
}