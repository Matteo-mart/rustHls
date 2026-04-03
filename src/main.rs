mod utils;
mod execute;
mod redis_store;

#[tokio::main] 
async fn main() {
    let video    = utils::arg_commande::arg_commande().unwrap_or_default();
    let dossier  = "tmp_result".to_string();
    let playlist = format!("{dossier}/playlist.m3u8");

    // Lance Redis en arrière-plan
    let output = std::process::Command::new("/usr/sbin/redis-server")
        .args([
            "--port", "6379", 
            "--daemonize", "yes"
        ])
        .output()
        .expect("Impossible de lancer le processus redis-server");

    if !output.status.success() {
        eprintln!("[redis] echec du lancement: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    // Prépare le dossier et fais la conversion
    let erreurs: Vec<String> = [
        utils::utils::utils(&dossier).map_err(|e| format!("[utils] {e}")),
        execute::convert_to_hls::convert_to_hls(&video, &dossier, &playlist).map_err(|e| format!("[hls] {e}")),
    ]
    .iter()
    .filter_map(|r| r.as_ref().err().cloned())
    .collect();

    println!("\nVidéo: '{video}'\n\nDossier: '{dossier}'\n");

    if erreurs.is_empty() {
        redis_store::set("playlist", &playlist).await.ok();
        redis_store::set("chemin_video", &video).await.ok();
        println!("[redis] OK");
    } else {
        erreurs.iter().for_each(|e| eprintln!("{e}"));
    }
}