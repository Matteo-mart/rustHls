use crate::execute::ffprobe::{ffprobe};
use crate::execute::ffmpeg::ffmpeg;
use crate::execute::{modifier_playlist};

pub fn convert_to_hls(chemin_video: &str, file_tmp_result: &str, chemin_playlist: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    ffprobe(chemin_video)?; 
    
    let base_name = chemin_video;
    
    ffmpeg(
        &[(
            chemin_video.to_string(),
            base_name.to_string()
        )], 
        file_tmp_result
    )?;  

    modifier_playlist::modifier_playlist(chemin_playlist, chemin_video)?;

    Ok(())
}