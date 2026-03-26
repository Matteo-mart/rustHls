use std::process::Command;
use serde_json;
use crate::utils::struct_types::{Stream, FFprobeOutput};

/// Commande FFprobe
pub fn ffprobe(file: &str) {
    println!("\nAnalyse de {} avec FFprobe", file);
    Command::new("ffprobe")
        .args(["-v", "error", "-i", file, "-print_format", "json", "-show_streams"])
        .output()
        .expect("Erreur sur la commande FFprobe");
    println!("\nFFprobe réussie");
}

/// Récupère les streams via FFprobe
pub fn get_streams(file: &str) -> Vec<Stream> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-i", file,
            "-print_format", "json",
            "-show_streams",
        ])
        .output()
        .expect("Erreur FFprobe");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: FFprobeOutput = serde_json::from_str(&stdout)
        .expect("Erreur : impossible de parser le JSON FFprobe");

    parsed.streams.iter().map(|s| Stream {
        codec_type: s.codec_type.clone(),
        lang: s.tags.get("language").cloned(),
        is_description: s.disposition.descriptions.unwrap_or(0) == 1,
    }).collect()
}

/// Détecte la hauteur vidéo via FFprobe
pub fn get_video_height(file: &str) -> u32 {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=height",
            "-of", "csv=p=0",
            file,
        ])
        .output()
        .expect("Erreur : impossible de lancer FFprobe");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.trim().parse::<u32>().unwrap_or(0)
}

/// Retourne la qualité selon la hauteur vidéo
pub fn get_quality(file: &str) -> &'static str {
    match get_video_height(file) {
        h if h >= 1080 => "hd",
        h if h >= 480  => "sd",
        _              => "md",
    }
}
