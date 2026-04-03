use std::process::Command;
use crate::utils::struct_types::{Stream, FFprobeOutput};

pub fn ffprobe(video: &str) -> Result<FFprobeOutput, Box<dyn std::error::Error>> {
    
    let output = Command::new("ffprobe")
        .args([
            "-v", "error", 
            "-i", video, 
            "-print_format", "json", 
            "-show_streams"
        ])
        .output()
        .map_err(|e| format!("Erreur lors du lancement de ffprobe : {}", e))?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FFprobe a échoué pour: '{}'\n {}", video, err_msg).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed = serde_json::from_str(&stdout)?;
    
    Ok(parsed)
}

pub fn get_streams(file: &str) -> Result<Vec<Stream>, Box<dyn std::error::Error>> {
    let output = ffprobe(file)?;
    Ok(output.streams)
}