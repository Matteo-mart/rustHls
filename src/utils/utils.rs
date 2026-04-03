use crate::utils;

/// appel les fonctions utiles pour la création et la suppression
pub fn utils(dossier: &str) -> Result<(), Box<dyn std::error::Error>> {
    utils::delete::delete(dossier)?;
    utils::create::create(dossier)?;
    Ok(())
}