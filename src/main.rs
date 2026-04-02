mod utils;
mod execute;

fn main() {

    let chemin_video = utils::arg_commande::arg_commande().unwrap_or_default();
    // let chemin_video = "video/fhd.mp4";
    let file_tmp_result = "tmp_result";
    let chemin_playlist = format!("{}/playlist.m3u8", file_tmp_result);

    let resultats = [
        utils::utils::utils(file_tmp_result).map_err(|e| format!("[utils] {}", e)),
        execute::convert_to_hls::convert_to_hls(&chemin_video, file_tmp_result, &chemin_playlist)
            .map_err(|e| format!("[convert_to_hls] {}", e)),
    ];

    println!("\nVidéo utilisée:\n'{}'", chemin_video);
    println!("\nDossier résultat:\n'{}'", file_tmp_result);

    let erreurs: Vec<_> = resultats.iter()
        .filter_map(|r| r
        .as_ref()
        .err())
        .collect();
    
    if erreurs.is_empty() {
        println!("\nAucune erreur\n");
    } else {
        erreurs
            .iter()
            .for_each(|e| eprintln!("\nErreur: {}", e));
    }

    println!("\nFIN\n");

}
