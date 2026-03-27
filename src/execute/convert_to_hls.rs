use crate::execute::ffprobe::{ffprobe};
use crate::execute::ffmpeg::ffmpeg;

///Convertion HLS
pub fn convert_to_hls(chemin_video: &str, file_tmp_result: &str) {
    
    ffprobe(chemin_video);
    
    let base_name = chemin_video;
    
    ffmpeg(
        &[(
            chemin_video.to_string(),
            base_name.to_string()
        )], 
        file_tmp_result
    );  
}
