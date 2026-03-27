use crate::utils;

pub fn utils(file_tmp_result: &str) -> Result<(), Box<dyn std::error::Error>> {
    utils::arg_commande::arg_commande();

    let res_delete = utils::delete::delete(file_tmp_result);
    let res_create = utils::create::create(file_tmp_result);
    match(res_delete, res_create) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(e), _) => Err(e.into()),
        (_, Err(e)) => Err(e.into()),

    }
}