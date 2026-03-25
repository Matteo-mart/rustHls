use crate::execute::convert_to_hls::convert_to_hls;
use crate::execute::modifier_playlist::modifier_playlist;

/// **Orchestre la conversion complète d'une source vidéo vers le format HLS.**
/// 
/// Cette fonction exécute deux étapes majeures :
/// 1. Appelle `convert_to_hls` pour générer les segments et la playlist via FFmpeg.
/// 2. Appelle `modifier_playlist` pour ajuster les tags HLS (notamment l'audiodescription).
/// 
/// **Retourne** : Le chemin vers le fichier de playlist généré en cas de succès.
pub fn convert(video_source: &str, directory_output: &str) -> Result<String, String> {
    println!("**********************************************");
    println!("Traitement du fichier : {}", video_source);

    let out = convert_to_hls(&[video_source.to_string()], directory_output)
        .map_err(|e| format!("Erreur lors de la conversion : {}", e))?;

    println!("FFmpeg a terminé. Playlist générée : {}", out.playlist_file);

    modifier_playlist(&out.playlist_file, video_source)
        .map_err(|e| format!("Erreur lors de la modification M3U8 : {}", e))?;

    println!("Modification de la playlist terminée.");

    Ok(out.playlist_file)
}