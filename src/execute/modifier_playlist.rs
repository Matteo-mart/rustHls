use std::fs;
use crate::execute::ffprobe;

pub fn modifier_playlist(path: &str, source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let streams = ffprobe::get_streams(source);
    
    let is_audio_ad = |lang: &str| -> bool {
        streams.iter().any(|s| {
            s.codec_type == "audio" && 
            s.tags.get("language").map_or("und", |l| l) == lang &&
            (s.disposition.default == 1 || s.disposition.captions == 1 || s.disposition.descriptions == 1)
        })
    };

    let contenu = fs::read_to_string(path)?;
    let mut lignes: Vec<String> = contenu.lines().map(String::from).collect();
    let mut modifie = false;

    for ligne in lignes.iter_mut().filter(|l| l.starts_with("#EXT-X-MEDIA:TYPE=AUDIO")) {
        let lang = extraire(ligne, "LANGUAGE");
        let ad = is_audio_ad(&lang);

        if ad {
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
    ligne.split(&format!("{}=\"", attr))
        .nth(1)
        .and_then(|s| s.split('"').next())
        .unwrap_or_default()
        .to_string()
}

fn remplacer(ligne: &str, attr: &str, nv_valeur: &str) -> String {
    let ancienne = extraire(ligne, attr);
    ligne.replace(&format!("{}=\"{}\"", attr, ancienne), &format!("{}=\"{}\"", attr, nv_valeur))
}