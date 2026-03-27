use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;

///commande FFmpeg
pub fn ffmpeg(videos: &[(String, String)], out_dir: &str) {

    //création dossiers
    let streams_path = Path::new(out_dir).join("streams");
    fs::create_dir_all(&streams_path).expect("Erreur création dossiers");

    //terminal propre
    let mut args = vec!["-hide_banner", "-loglevel", "error"].into_iter().map(String::from).collect::<Vec<_>>();
    let mut map_args = vec![];
    let mut stream_maps: Vec<String> = vec![];
    
    let (mut video_idx, mut audio_idx) = (0, 0);

    //construction des arguments
    for (input_idx, (path, base_name)) in videos.iter().enumerate() {
        args.extend(["-i".to_string(), path.clone()]);

        let (mut local_video, mut local_audio) = (0, 0);

        for s in crate::execute::ffprobe::get_streams(path) {
            let lang = s.tags.get("language").map(|s| s.as_str()).unwrap_or("und");

            match s.codec_type.as_str() {
                "video" => {
                    map_args.extend(["-map".into(), format!("{}:v:{}", input_idx, local_video)]);
                    
                    let desc = if s.disposition.descriptions == 1 { ",characteristics:public.accessibility.describes-video" } else { "" };
                    stream_maps.push(format!("v:{},agroup:{},name:v_{}_{}{}", video_idx, base_name, lang, video_idx, desc));
                    
                    video_idx += 1; local_video += 1;
                }
                "audio" => {
                    map_args.extend(["-map".into(), format!("{}:a:{}", input_idx, local_audio)]);
                    stream_maps.push(format!("a:{},agroup:{},name:a_{}_{},language:{}", audio_idx, base_name, lang, audio_idx, lang));
                    
                    audio_idx += 1; local_audio += 1;
                }
                _ => {}
            }
        }
    }

    //commande finale
    args.extend(["-c".into(), "copy".into()]);
    args.extend(map_args);
    args.extend([
        "-f".into(), "hls".into(),
        "-var_stream_map".into(), stream_maps.join(" "),
        "-hls_flags".into(), "round_durations+independent_segments".into(),
        "-hls_list_size".into(), "0".into(),
        "-hls_time".into(), "5".into(),
        "-master_pl_name".into(), "playlist.m3u8".into(),
        "-hls_segment_filename".into(), format!("{}/streams/%v_%03d.ts", out_dir),
        format!("{}/%v.m3u8", out_dir)
    ]);

    //execution
    let success = Command::new("ffmpeg")
        .args(&args)
        .stderr(Stdio::inherit())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !success {
        eprintln!("\nErreur de conversion FFmpeg\n");
        std::process::exit(1);
    }
}