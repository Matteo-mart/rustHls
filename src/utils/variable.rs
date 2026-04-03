use crate::utils::{affichage, arg_commande};

/// définis les var utilisées
pub fn variable() -> (String, String, String) {
    let video    = arg_commande::arg_commande().unwrap_or_default();
    let dossier  = "tmp_result".to_string();
    let playlist = format!("{dossier}/playlist.m3u8");

    affichage::affichage(&dossier, &video);

    (video, dossier, playlist)
}

