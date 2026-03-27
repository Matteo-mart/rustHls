use std::fs;
use crate::execute::ffprobe;

pub fn modifier_playlist(path: &str, source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let streams = ffprobe::ffprobe(source)?.streams;
    
    let is_audio_ad = |lang: &str| -> bool {
        streams.iter().any(|s| {
            let s_lang = s.tags.get("language").map(|l| l.as_str()).unwrap_or("und");
            s.codec_type == "audio" && 
            s_lang == lang &&
            (s.disposition.default == 1 || s.disposition.captions == 1 || s.disposition.descriptions == 1)
        })
    };

    let contenu = fs::read_to_string(path).map_err(|e| {
        format!("modification_playlist: impossible de lire la playlist '{}': {}", path, e)
    })?;

    let mut lignes: Vec<String> = contenu.lines().map(String::from).collect();
    let mut modifie = false;

    for ligne in lignes.iter_mut().filter(|l| l.starts_with("#EXT-X-MEDIA:TYPE=AUDIO")) {
        let lang = extraire(ligne, "LANGUAGE");
        if is_audio_ad(&lang) {
            if !ligne.contains("CHARACTERISTICS") {
                *ligne = ligne.replace("NAME=", "CHARACTERISTICS=\"public.accessibility.describes-video\",NAME=");
            }
            *ligne = remplacer(ligne, "NAME", "AD");
            modifie = true;
        }
    }

    if modifie {
        fs::write(format!("{}.bak", path), &contenu)?;
        fs::write(path, lignes.join("\n"))?;
    }

    Ok(())
}

fn extraire(ligne: &str, attr: &str) -> String {
    ligne.split_once(&format!("{}=\"", attr))
        .and_then(|(_, reste)| reste.split('"').next())
        .unwrap_or_default()
        .to_string()
}

fn remplacer(ligne: &str, attr: &str, nv_valeur: &str) -> String {
    let ancienne = extraire(ligne, attr);
    ligne.replace(&format!("{}=\"{}\"", attr, ancienne), &format!("{}=\"{}\"", attr, nv_valeur))
}