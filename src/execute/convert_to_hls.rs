use crate::execute::ffprobe::{ffprobe, get_quality};
use crate::execute::ffmpeg::ffmpeg;
use crate::execute::modif_playlist;

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


    let chemin_m3u8 = "playlist.m3u8";

    modif_playlist::modif_playlist(chemin_m3u8);
}