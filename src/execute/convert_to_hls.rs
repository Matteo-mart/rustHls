use crate::execute::ffprobe::{ffprobe, get_quality};
use crate::execute::ffmpeg::ffmpeg;


///Convertion en HLS processus
pub fn convert_to_hls(file: &str) {
    ffprobe(file);
    let base_name = get_quality(file);

    ffmpeg(&[(
        file, 
        base_name
    )]);
}
