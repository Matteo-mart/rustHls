use std::process::Command;
use crate::utils::struct_types::{Stream, FFprobeOutput};

/// Commande FFprobe
pub fn ffprobe(chemin_video: &str) -> FFprobeOutput {
    
    let output = Command::new("ffprobe")
        .args([
            "-v", "error", 
            "-i", chemin_video, 
            "-print_format", "json", 
            "-show_streams"
        ])
        .output()
        .expect("\nErreur sur la commande FFprobe\n");

    let stdout = String::from_utf8_lossy(&output.stdout);

    serde_json::from_str(&stdout)
        .expect("\nErreur JSON FFprobe\n")
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
        .expect("\nErreur FFprobe\n");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let parsed: FFprobeOutput = serde_json::from_str(&stdout)
        .expect("\nErreur JSON FFprobe\n");

    parsed.streams
}