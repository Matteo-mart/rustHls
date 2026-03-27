mod utils;
mod execute;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::clear::clear();

    let chemin_video = utils::arg_commande::arg_commande();
    let file_tmp_result = "tmp_result";
    let chemin_playlist = format!("{}/playlist.m3u8", file_tmp_result);

    println!("\n--- Fichier géré: {} ---\n", chemin_video);
    println!("\n--- Dossier résultat: {} ---\n", file_tmp_result);

    utils::delete::delete(file_tmp_result)?;
    utils::create::create(file_tmp_result)?;
    execute::convert_to_hls::convert_to_hls(&chemin_video, file_tmp_result);
    execute::modifier_playlist::modifier_playlist(&chemin_playlist, &chemin_video)?;

    println!("\n--- FIN ---\n");
    Ok(())
}