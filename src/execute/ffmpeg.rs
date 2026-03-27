use std::process::Command;
use crate::execute::ffprobe;
use std::process::Stdio;

///Commande FFmpeg
pub fn ffmpeg(chemin_video: &[(&str, &str)], file_tmp_result: &str) {
    let master_playlist = "playlist.m3u8";

    for (_, base_name) in chemin_video.iter() {
        let sub_dir = format!("{}/{}", file_tmp_result, base_name);
        std::fs::create_dir_all(&sub_dir)
            .expect(&format!("\nImpossible de créer le dossier {}\n", sub_dir));
    }

    let mut input_args: Vec<String> = vec![];
    let mut map_args: Vec<String> = vec![];
    let mut stream_map_audio: Vec<String> = vec![];
    let mut stream_map_video: Vec<String> = vec![];
    let mut global_idx_audio = 0;
    let mut global_idx_video = 0;

    for (chemin_video_idx, (chemin_video, base_name)) in chemin_video.iter().enumerate() {
        input_args.push("-i".to_string());
        input_args.push(chemin_video.to_string());

        let streams = ffprobe::get_streams(chemin_video);
        let mut local_idx_audio = 0;
        let mut local_idx_video = 0;

        for stream in &streams {
            // Récupération langue depuis tags
            let lang = stream.tags.get("language")
                .filter(|l| !l.is_empty())
                .map(|l| l.as_str())
                .unwrap_or("und");

            // Récupération is_ad depuis disposition
            let is_ad = stream.disposition.descriptions == 1;

            match stream.codec_type.as_str() {
                "video" => {
                    map_args.push("-map".to_string());
                    map_args.push(format!("{}:v:{}", chemin_video_idx, local_idx_video));

                    let stream_name = format!("{}/v_{}", base_name, lang);
                    let desc = if is_ad {
                        ",characteristics:public.accessibility.describes-video"
                    } else {
                        ""
                    };

                    stream_map_video.push(format!(
                        "v:{},agroup:{},name:{}{}",
                        global_idx_video, base_name, stream_name, desc
                    ));
                    global_idx_video += 1;
                    local_idx_video += 1;
                }
                "audio" => {
                    map_args.push("-map".to_string());
                    map_args.push(format!("{}:a:{}", chemin_video_idx, local_idx_audio));

                    let stream_name = format!("{}/a_{}", base_name, lang);
                    stream_map_audio.push(format!(
                        "a:{},agroup:{},name:{},language:{}",
                        global_idx_audio, base_name, stream_name, lang
                    ));
                    global_idx_audio += 1;
                    local_idx_audio += 1;
                }
                _ => {}
            }
        }
    }

    let mut full_stream_map = stream_map_audio.clone();
    full_stream_map.extend(stream_map_video.clone());
    let full_stream_map = full_stream_map.join(" ");

    let segment_filename = format!("{}/%v_%03d.ts", file_tmp_result);
    let output_playlist = format!("{}/%v.m3u8", file_tmp_result);

    let mut args: Vec<String> = vec![];
    args.extend(input_args);
    args.extend(["-c".to_string(), "copy".to_string()]);
    args.extend(map_args);
    args.extend([
        "-f".to_string(), "hls".to_string(),
        "-var_stream_map".to_string(), full_stream_map,
        "-hls_flags".to_string(), "round_durations".to_string(),
        "-hls_list_size".to_string(), "0".to_string(),
        "-hls_time".to_string(), "5".to_string(),
        "-master_pl_name".to_string(), master_playlist.to_string(),
        "-hls_segment_filename".to_string(), segment_filename,
        output_playlist,
    ]);

    let status = Command::new("ffmpeg")
        .args(&["-hide_banner", "-loglevel", "error"])
        .args(&args)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status()
        .expect("\nErreur : impossible de lancer FFmpeg\n");

    if !status.success() {
        eprintln!("\nErreur de conversion FFmpeg\n");
        std::process::exit(1);
    }
}