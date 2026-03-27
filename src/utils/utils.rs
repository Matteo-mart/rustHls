use crate::utils;

pub fn utils(file_tmp_result: &str) -> Result<(), Box<dyn std::error::Error>> {

    utils::clear::clear();
    utils::arg_commande::arg_commande();
    utils::delete::delete(file_tmp_result)?;
    utils::create::create(file_tmp_result)?;

    Ok(())
}