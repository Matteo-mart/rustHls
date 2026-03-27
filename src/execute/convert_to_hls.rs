use crate::execute::ffprobe::{ffprobe, get_quality};
use crate::execute::ffmpeg::ffmpeg;

/// Conversion en HLS processus
pub fn convert_to_hls(chemin_video: &str, file_tmp_result: &str) {
    
    ffprobe(chemin_video);
    
    let base_name = get_quality(chemin_video);
    
    ffmpeg(
        &[(
            chemin_video, 
            &base_name
        )], 
        file_tmp_result
    );  
}