mod utils;
mod execute;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //variable
    let chemin_video = utils::arg_commande::arg_commande();
    let file_tmp_result = "/hhs";
    let chemin_playlist = format!("{}/playlist.m3u8", file_tmp_result);
    //fonction
    let _ = utils::utils::utils(file_tmp_result);
    execute::convert_to_hls::convert_to_hls(&chemin_video, file_tmp_result, &chemin_playlist);

    println!("\nDossier résultat:\n'{}'", file_tmp_result);
    // println!("\n ")
    Ok(())
}