use std::fs;

pub fn create(file_tmp_result: &str) -> std::io::Result<()> {
    fs::create_dir(file_tmp_result).map_err(|e| {
        eprintln!("\nErreur création du dossier '{}': {}\n", file_tmp_result, e); e})?;
    Ok(())
}