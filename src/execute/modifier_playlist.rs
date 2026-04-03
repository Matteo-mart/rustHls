use std::fs;
use crate::execute::ffprobe;

///modification de la playlist pour marque les AD
pub fn modifier_playlist(path: &str, source: &str) -> Result<(), Box<dyn std::error::Error>> {

    // recupere les streams du fichier
    let streams = ffprobe::ffprobe(source)?.streams;

    //verifie si c'est un AD
    let is_audio_ad = |lang: &str| -> bool {
        streams.iter().any(|s| {
            let s_lang = s.tags.get("language").map(|l| l.as_str()).unwrap_or("und");
            s.codec_type == "audio" &&
            s_lang == lang &&
            (s.disposition.default == 1 || s.disposition.captions == 1 || s.disposition.descriptions == 1)
        })
    };

    // lit le contenu brut de la playlist.m3u8
    let contenu = fs::read_to_string(path).map_err(|e| {
        format!("modification_playlist: impossible de lire la playlist '{}': {}", path, e)
    })?;

    let mut lignes: Vec<String> = contenu.lines().map(String::from).collect();
    let mut modifie = false;

    // prend en compte que les lignes avec: '#EXT-X-MEDIA:TYPE=AUDIO'
    for ligne in lignes.iter_mut().filter(|l| l.starts_with("#EXT-X-MEDIA:TYPE=AUDIO")) {
        let lang = extraire(ligne, "LANGUAGE");

        if is_audio_ad(&lang) {
            // ajoute l'attribut CHARACTERISTICS si pas la
            if !ligne.contains("CHARACTERISTICS") {
                *ligne = ligne.replace(
                    "NAME=",
                    "CHARACTERISTICS=\"public.accessibility.describes-video\",NAME="
                );
            }

            // renomme la piste en "AD" la reconnaître
            *ligne = remplacer(ligne, "NAME", "AD");
            modifie = true;
        }
    }

    if modifie {
        // sauvegarde une copie de la playlist
        fs::write(format!("{}.bak", path), &contenu)?;
        fs::write(path, lignes.join("\n"))?;
    }

    Ok(())
}

/// extrait la valeur d'un attribut dans une ligne de fichier .m3u8
fn extraire(ligne: &str, attr: &str) -> String {
    ligne.split_once(&format!("{}=\"", attr))
        .and_then(|(_, reste)| reste.split('"').next())
        .unwrap_or_default()
        .to_string()
}

/// remplace la valeur d'un attribut dans un fichier .m3u8
fn remplacer(ligne: &str, attr: &str, nv_valeur: &str) -> String {
    let ancienne = extraire(ligne, attr);
    ligne.replace(
        &format!("{}=\"{}\"", attr, ancienne),
        &format!("{}=\"{}\"", attr, nv_valeur)
    )
}