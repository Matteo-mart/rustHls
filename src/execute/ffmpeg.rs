use std::{process::Command};
use crate::execute::ffprobe;

///Commande FFmpeg
pub fn ffmpeg(files: &[(&str, &str)]) {
    // println!("Commande FFmpeg");

    // let directory_out = "tmp_result";
    let master_playlist = "playlist.m3u8";


    for (_, base_name) in files.iter() {
        // let sub_dir = format!("{}/{}", directory_out, base_name);
        let sub_dir = format!("tmp_result/{}", base_name);
        std::fs::create_dir_all(&sub_dir)
            .expect(&format!("Impossible de créer le dossier {}", sub_dir));
    }

    let mut input_args: Vec<String> = vec![];
    let mut map_args: Vec<String> = vec![];
    let mut stream_map_audio: Vec<String> = vec![];
    let mut stream_map_video: Vec<String> = vec![];
    let mut global_idx_audio = 0;
    let mut global_idx_video = 0;

    for (file_idx, (file, base_name)) in files.iter().enumerate() {
        input_args.push("-i".to_string());
        input_args.push(file.to_string());

        let streams = ffprobe::get_streams(file);
        let mut local_idx_audio = 0;
        let mut local_idx_video = 0;


        for stream in &streams {
            let lang = stream.lang.as_deref().unwrap_or("und");
            
            match stream.codec_type.as_str() {
                "video" => {
                    map_args.push("-map".to_string());
                    map_args.push(format!("{}:v:{}", file_idx, local_idx_video));

                    let stream_name = format!("{}/v_{}", base_name, lang);
                    let desc = if stream.is_description {
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
                    map_args.push(format!("{}:a:{}", file_idx, local_idx_audio));

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



    // Audio en premier, puis vidéo (comme le Go)
    let mut full_stream_map = stream_map_audio.clone();
    full_stream_map.extend(stream_map_video.clone());
    let full_stream_map = full_stream_map.join(" ");

    // let segment_filename = format!("{}/%v_%03d.ts", directory_out);
    // let output_playlist = format!("{}/%v.m3u8", directory_out);

    let segment_filename = format!("tmp_result/%v_%03d.ts");
    let output_playlist = format!("tmp_result/%v.m3u8");

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

    println!("\nCOMMANDE : ffmpeg {}\n", args.join(" "));

    let status = Command::new("ffmpeg")
        .args(&args)
        .status()
        .expect("Erreur : impossible de lancer FFmpeg");

    if !status.success() {
        eprintln!("Erreur de conversion FFmpeg");
        std::process::exit(1);
    }
}