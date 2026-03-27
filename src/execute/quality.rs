use std::process::Command;

/// Détecte la hauteur vidéo via FFprobe
pub fn get_video_height(chemin_video: &str) -> u32 {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=height",
            "-of", "csv=p=0",
            chemin_video,
        ])
        .output()
        .expect("\nErreur : impossible de lancer FFprobe\n");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.trim().parse::<u32>().unwrap_or(0)
}

/// Retourne la qualité selon la hauteur vidéo
pub fn get_quality(chemin_video: &str) -> &'static str {
    match get_video_height(chemin_video) {
        h if h >= 1080 => "hd",
        h if h >= 480  => "sd",
        _              => "md",
    }
}
