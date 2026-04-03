mod utils;
mod execute;

use utils::{variable, redis};

#[tokio::main]
async fn main() {
    let (video, dossier, playlist) = variable::variable();
    variable::demarrer_redis();

    let erreurs: Vec<String> = [
        utils::utils::utils(&dossier)
            .map_err(|e| format!("[utils] {e}")),
        execute::convert_to_hls::convert_to_hls(&video, &dossier, &playlist)
            .map_err(|e| format!("[convert_to_hls] {e}")),
    ]
    .iter()
    .filter_map(|r| r.as_ref().err().cloned())
    .collect();

    if erreurs.is_empty() {
        redis::set("playlist", &playlist).await.ok();
        redis::set("video", &video).await.ok();
        println!("\nSuccès\n");
    } else {
        erreurs.iter().for_each(|e| eprintln!("{e}"));
    }
}