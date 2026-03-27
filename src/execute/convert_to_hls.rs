use crate::execute::ffprobe::{ffprobe};
use crate::execute::ffmpeg::ffmpeg;
use crate::execute::{modifier_playlist};

///Convertion HLS
pub fn convert_to_hls(chemin_video: &str, file_tmp_result: &str, chemin_playlist: &str) {
    
    ffprobe(chemin_video);
    
    let base_name = chemin_video;
    
    ffmpeg(
        &[(
            chemin_video.to_string(),
            base_name.to_string()
        )], 
        file_tmp_result
    );  

    let _ = modifier_playlist::modifier_playlist(chemin_playlist, chemin_video);
}
