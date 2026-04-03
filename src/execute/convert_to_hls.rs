use crate::execute::ffprobe::{ffprobe};
use crate::execute::ffmpeg::ffmpeg;
use crate::execute::{modifier_playlist};

/// Appel des fonctions pour la conversion
pub fn convert_to_hls(video: &str, dossier: &str, playlist: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    ffprobe(video)?; 
    
    let base_name = video;
    
    ffmpeg(&[(
            video.to_string(),
            base_name.to_string()
        )],
        dossier)?;  

    modifier_playlist::modifier_playlist(playlist, video)?;

    Ok(())
}