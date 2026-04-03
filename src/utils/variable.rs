use crate::utils::{affichage, arg_commande};

/// définis les var utilisées
pub fn variable() -> (String, String, String) {
    let video    = arg_commande::arg_commande().unwrap_or_default();
    let dossier  = "tmp_result".to_string();
    let playlist = format!("{dossier}/playlist.m3u8");

    affichage::affichage(&dossier, &video);

    (video, dossier, playlist)
}

/// démarre redis
pub fn demarrer_redis() {
    let output = std::process::Command::new("/usr/sbin/redis-server")
        .args(["--port", "6379", "--daemonize", "yes"])
        .output()
        .expect("Impossible de lancer redis-server");

    if !output.status.success() {
        eprintln!(
            "Échec du lancement Redis : {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }
}