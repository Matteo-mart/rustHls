mod utils;
mod execute;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chemin_video = utils::arg_commande::arg_commande();
    let file_tmp_result = "tmp_result";
    let chemin_playlist = format!("{}/playlist.m3u8", file_tmp_result);

    let _ = utils::utils::utils(file_tmp_result);
    execute::convert_to_hls::convert_to_hls(&chemin_video, file_tmp_result, &chemin_playlist);

    println!("\n--- Fichier géré: {} ---\n", chemin_video);
    println!("\n--- Dossier résultat: {} ---\n", file_tmp_result);
    Ok(())
}