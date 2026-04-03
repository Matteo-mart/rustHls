use crate::utils;

pub fn utils(dossier: &str) -> Result<(), Box<dyn std::error::Error>> {
    utils::delete::delete(dossier)?;
    utils::create::create(dossier)?;
    Ok(())
}