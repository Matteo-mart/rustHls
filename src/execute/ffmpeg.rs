use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;

/// Commande FFmpeg corrigée
pub fn ffmpeg(videos: &[(String, String)], file_tmp_result: &str) {
    let master_playlist = "playlist.m3u8";

    // 1. Création des sous-répertoires
    for (_, base_name) in videos {
        let sub_dir = Path::new(file_tmp_result).join(base_name);
        fs::create_dir_all(&sub_dir)
            .unwrap_or_else(|_| panic!("\nImpossible de créer le dossier {:?}\n", sub_dir));
    }

    let mut input_args: Vec<String> = vec![];
    let mut map_args: Vec<String> = vec![];
    let mut stream_maps: Vec<String> = vec![];
    
    let mut global_idx_video = 0;
    let mut global_idx_audio = 0;

    // 2. Construction des arguments par fichier
    for (input_idx, (chemin_video, base_name)) in videos.iter().enumerate() {
        input_args.push("-i".to_string());
        input_args.push(chemin_video.clone());

        // Note: Assure-toi que ffprobe::get_streams retourne bien une structure compatible
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
                    // Format: index_fichier:v:index_local_video
                    map_args.push(format!("{}:v:{}", input_idx, local_idx_video));

                    let is_ad = stream.disposition.descriptions == 1;
                    let desc = if is_ad { ",characteristics:public.accessibility.describes-video" } else { "" };
                    
                    // On définit le flux v:global_idx avec son agroup (nom du dossier/film)
                    stream_maps.push(format!(
                        "v:{},agroup:{},name:v_{}_{}{}",
                        global_idx_video, base_name, lang, input_idx, desc
                    ));
                    
                    global_idx_video += 1;
                    local_idx_video += 1;
                }
                "audio" => {
                    map_args.push("-map".to_string());
                    map_args.push(format!("{}:a:{}", input_idx, local_idx_audio));

                    stream_maps.push(format!(
                        "a:{},agroup:{},name:a_{}_{},language:{}",
                        global_idx_audio, base_name, lang, input_idx, lang
                    ));
                    
                    global_idx_audio += 1;
                    local_idx_audio += 1;
                }
                _ => {}
            }
        }
    }

    let full_stream_map = stream_maps.join(" ");

    // 3. Configuration de la sortie HLS
    let mut args: Vec<String> = vec![];
    args.extend(["-hide_banner".to_string(), "-loglevel".to_string(), "error".to_string()]);
    args.extend(input_args);
    args.extend(["-c".to_string(), "copy".to_string()]); // Remuxing sans ré-encodage
    args.extend(map_args);
    
    args.extend([
        "-f".to_string(), "hls".to_string(),
        "-var_stream_map".to_string(), full_stream_map,
        "-hls_flags".to_string(), "round_durations+independent_segments".to_string(),
        "-hls_list_size".to_string(), "0".to_string(),
        "-hls_time".to_string(), "5".to_string(),
        "-master_pl_name".to_string(), master_playlist.to_string(),
        // %v sera remplacé par l'index du variant (0, 1, 2...) défini dans var_stream_map
        "-hls_segment_filename".to_string(), format!("{}/%v_%03d.ts", file_tmp_result),
    ]);
    
    // Le fichier de sortie final pour les playlists variantes
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