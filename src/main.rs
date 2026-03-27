mod utils;
mod execute;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chemin_video = utils::arg_commande::arg_commande();
    let file_tmp_result = "tmp_result"; 
    let chemin_playlist = format!("{}/playlist.m3u8", file_tmp_result);

    let status_utils = utils::utils::utils(file_tmp_result);
    
    let status_hls = execute::convert_to_hls::convert_to_hls(&chemin_video, file_tmp_result, &chemin_playlist);

    println!("\nDossier résultat:\n'{}'", file_tmp_result);
    println!("\nCOUCOU\n");

    status_utils?;
    status_hls?;

    Ok(())
}