use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;

/// Commande FFmpeg configurée pour organiser les fichiers
pub fn ffmpeg(videos: &[(String, String)], file_tmp_result: &str) {
    let master_playlist = "playlist.m3u8";

    let output_path = Path::new(file_tmp_result);
    let streams_path = output_path.join("streams");

    if !output_path.exists() {
        fs::create_dir_all(output_path).expect("Impossible de créer le dossier de sortie");
    }
    if !streams_path.exists() {
        fs::create_dir_all(&streams_path).expect("Impossible de créer le dossier des segments");
    }

    let mut input_args: Vec<String> = vec![];
    let mut map_args: Vec<String> = vec![];
    let mut stream_maps: Vec<String> = vec![];
    
    let mut global_idx_video = 0;
    let mut global_idx_audio = 0;

    for (input_idx, (chemin_video, base_name)) in videos.iter().enumerate() {
        input_args.push("-i".to_string());
        input_args.push(chemin_video.clone());

        let streams = crate::execute::ffprobe::get_streams(chemin_video);
        
        let mut local_idx_video = 0;
        let mut local_idx_audio = 0;

        for stream in &streams {
            let lang = stream.tags.get("language")
                .map(|s| s.as_str())
                .unwrap_or("und");

            match stream.codec_type.as_str() {
                "video" => {
                    map_args.push("-map".to_string());
                    map_args.push(format!("{}:v:{}", input_idx, local_idx_video));

                    let is_ad = stream.disposition.descriptions == 1;
                    let desc = if is_ad { ",characteristics:public.accessibility.describes-video" } else { "" };
                    
                    stream_maps.push(format!(
                        "v:{},agroup:{},name:v_{}_{}{}",
                        global_idx_video, base_name, lang, global_idx_video, desc
                    ));
                    
                    global_idx_video += 1;
                    local_idx_video += 1;
                }
                "audio" => {
                    map_args.push("-map".to_string());
                    map_args.push(format!("{}:a:{}", input_idx, local_idx_audio));

                    stream_maps.push(format!(
                        "a:{},agroup:{},name:a_{}_{},language:{}",
                        global_idx_audio, base_name, lang, global_idx_audio, lang
                    ));
                    
                    global_idx_audio += 1;
                    local_idx_audio += 1;
                }
                _ => {}
            }
        }
    }

    let full_stream_map = stream_maps.join(" ");

    let mut args: Vec<String> = vec![];
    args.extend(["-hide_banner".to_string(), "-loglevel".to_string(), "error".to_string()]);
    args.extend(input_args);    
    args.extend(["-c".to_string(), "copy".to_string()]); 
    args.extend(map_args);
    args.extend([
        "-f".to_string(), "hls".to_string(),
        "-var_stream_map".to_string(), full_stream_map,
        "-hls_flags".to_string(), "round_durations+independent_segments".to_string(),
        "-hls_list_size".to_string(), "0".to_string(),
        "-hls_time".to_string(), "5".to_string(),
        "-master_pl_name".to_string(), master_playlist.to_string(),        
        "-hls_segment_filename".to_string(), format!("{}/streams/%v_%03d.ts", file_tmp_result),
    ]);
    args.push(format!("{}/%v.m3u8", file_tmp_result));
    let status = Command::new("ffmpeg")
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