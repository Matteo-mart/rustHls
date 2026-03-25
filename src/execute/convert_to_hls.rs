use std::fs;
use std::process::Command;

use super::affichage::affichage_file_test;
use super::segment::file_name_without_extension;
use super::ffprobe::ffprobe;
use crate::utils::struct_types::ConvertToHlsOutput;



// param:
// - input_files : la liste des fichiers vidéo à convertir
// - directory_out : le dossier où seront stockés les fichiers générés

// retourne sois chemin playlist soit erreur
///Fonction qui convertit des fichiers vidéo en format HLS
pub fn convert_to_hls(
    input_files: &[String],
    directory_out: &str,
) -> Result<ConvertToHlsOutput, String> {

    println!("--- convertion ---");

    affichage_file_test("tmp_result");

    if input_files.is_empty() {
        return Err("\nAucune vidéo fournie".to_string());
    }

    let first_base_name = file_name_without_extension(&input_files[0]);

    let master_playlist_name = format!("{}Playlist.m3u8", first_base_name);
    println!("\nTraitement HLS pour {:?}", input_files);

    let mut input_args: Vec<String> = Vec::new();   // arguments pour les fichiers entrée
    let mut map_args: Vec<String> = Vec::new();      // arguments pour le map des flux
    let mut stream_map_audio: Vec<String> = Vec::new(); // liste des flux audio
    let mut stream_map_video: Vec<String> = Vec::new(); // liste des flux vidéo

    // compteurs pr indexe les flux
    let mut global_idx_audio = 0;
    let mut global_idx_video = 0;

    for (file_idx, file) in input_files.iter().enumerate() {

        let base_name = file_name_without_extension(file);

        let sub_dir = format!("{}/{}", directory_out, base_name);

        fs::create_dir_all(&sub_dir)
            .map_err(|e| format!("\nErreur création dossier {} : {}", sub_dir, e))?;

        let res = ffprobe(file)?;

        input_args.push("-i".to_string());
        input_args.push(file.clone());

        let mut local_idx_audio = 0;
        let mut local_idx_video = 0;

        for stream in &res.streams {

            let lang = stream.tags.get("language").cloned().unwrap_or_default();

            match stream.codec_type.as_str() {

                "video" => {
                    map_args.push("-map".to_string());
                    map_args.push(format!("{}:v:{}", file_idx, local_idx_video));

                    let stream_name = format!("{}/v_{}", base_name, lang);

                    let desc = if stream.disposition_descriptions == 1 {
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

    let full_stream_map = stream_map_audio
        .iter()
        .chain(stream_map_video.iter())
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");

    let mut args: Vec<String> = Vec::new();
    args.extend(input_args);
    args.extend(["-c".to_string(), "copy".to_string()]);
    args.extend(map_args);
    args.extend([
        "-f".to_string(), "hls".to_string(),// format de sortie HLS
        "-var_stream_map".to_string(), full_stream_map,// mapping variant
        "-hls_flags".to_string(), "round_durations".to_string(), // durées
        "-hls_list_size".to_string(), "0".to_string(),  // garde tous les segments
        "-hls_time".to_string(), "5".to_string(), // segments de 5 secondes
        "-master_pl_name".to_string(), master_playlist_name.clone(), // nom de la playlist
        "-hls_segment_filename".to_string(), format!("{}/%v_%03d.ts", directory_out), // nom des segments
        format!("{}/%v.m3u8", directory_out), // playlists de variants
    ]);

    println!("\nCOMMANDE: ffmpeg {}\n\n", args.join(" "));

    let output = Command::new("ffmpeg")
        .args(&args)
        .output()
        .map_err(|e| format!("Échec lancement ffmpeg : {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Échec ffmpeg | Output: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("FFMPEG résultat : Succès");

    Ok(ConvertToHlsOutput {
        playlist_file: format!("{}/{}", directory_out, master_playlist_name),
    })
}