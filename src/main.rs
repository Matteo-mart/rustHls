use crate::utils::affichage;

mod utils;
mod execute;

#[tokio::main] 
async fn main() {
    
    // récupère le chemin de la vidéo passé en argument dans la commande
    let video    = utils::arg_commande::arg_commande().unwrap_or_default();
    let dossier  = "tmp_result".to_string();
    let playlist = format!("{dossier}/playlist.m3u8");

    affichage::affichage(&dossier, &video);

    // lance redis en arrière-plan, sinon bloqué à output indéfiniment
    let output = std::process::Command::new("/usr/sbin/redis-server")
        .args([
            "--port", "6379", 
            "--daemonize", "yes"
        ])
        .output()
        .expect("Impossible de lancer le processus redis-server");

    if !output.status.success() {
        eprintln!("echec du lancement du serveur: {}",  
            String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    // prépare le dossier de sortie et lance la conversion
    let erreurs: Vec<String> = [
        utils::utils::utils(&dossier).map_err(|e| format!("[utils] {e}")),
        execute::convert_to_hls::convert_to_hls(&video, &dossier, &playlist).map_err(|e| format!("[convert_to_hls] {e}")),
    ]
    .iter()
    .filter_map(|r| r.as_ref().err().cloned())
    .collect();

    // stocke les chemins dans 'Redis' si tout c'est bien passé
    if erreurs.is_empty() {
        utils::redis::set("playlist", &playlist).await.ok();
        utils::redis::set("video", &video).await.ok();
        println!("\nSuccès\n");
    } else {
        erreurs.iter().for_each(|e| eprintln!("{e}"));
    }

}