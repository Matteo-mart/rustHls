use std::process::Command;
use serde_json;
use crate::utils::struct_types::FfprobeOutput;

///lancement de ffprobe
pub fn ffprobe(file: &str) -> Result<FfprobeOutput, String> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-i", file,
            "-print_format", "json",
            "-show_streams",
        ])
        .output()
        .map_err(|e| format!("Erreur lancement ffprobe : {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Échec ffprobe : {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let result: FfprobeOutput = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Erreur parsing JSON ffprobe : {}", e))?;

    Ok(result)
}