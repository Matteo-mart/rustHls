use crate::execute::convert_to_hls::convert_to_hls;
use crate::execute::modifier_playlist::modifier_playlist;

pub fn convert(video_source: &str, directory_output: &str) -> Result<String, String> {
    println!("**********************************************");
    println!("Traitement du fichier : {}", video_source);

    let out = convert_to_hls(&[video_source.to_string()], directory_output)
        .map_err(|e| format!("Erreur lors de la conversion : {}", e))?;

    println!("FFmpeg a terminé. Playlist générée : {}", out.playlist_file);


    modifier_playlist(&out.playlist_file, video_source)
        .map_err(|e| format!("Erreur lors de la modification M3U8 : {}", e))?;

    println!("Modification de la playlist terminée.");

    // 3. Retourne le chemin final
    Ok(out.playlist_file)
}