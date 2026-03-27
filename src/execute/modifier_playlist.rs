use std::fs;
use std::collections::HashMap;
use crate::execute::ffprobe;

pub fn modifier_playlist(chemin_m3u8: &str, video_source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let streams = ffprobe::get_streams(video_source);

    let mut audio_map: HashMap<String, bool> = HashMap::new();
    for s in &streams {
        if s.codec_type == "audio" {
            let lang = s.tags.get("language")
                .filter(|l| !l.is_empty())
                .cloned()
                .unwrap_or("und".to_string());

            let is_ad = s.disposition.default == 1
                || s.disposition.captions == 1
                || s.disposition.descriptions == 1;
            
            audio_map.insert(lang, is_ad);
        }
    }

    let contenu = fs::read_to_string(chemin_m3u8)?;
    let mut lignes: Vec<String> = contenu.lines().map(String::from).collect();
    let mut modifie = false;

    for i in 0..lignes.len() {
        if lignes[i].starts_with("#EXT-X-MEDIA:TYPE=AUDIO") {
            let lang = extraire(&lignes[i], "LANGUAGE");
            let is_ad = *audio_map.get(&lang).unwrap_or(&false);

            if is_ad && !lignes[i].contains("CHARACTERISTICS") {
                lignes[i] = lignes[i].replace(
                    "NAME=",
                    r#"CHARACTERISTICS="public.accessibility.describes-video",NAME="#
                );
            }

            let nouveau_name = if is_ad { "AD".to_string() } else { lang };
            lignes[i] = remplacer(&lignes[i], "NAME", &nouveau_name);
            modifie = true;
        }
    }

    if modifie {
        fs::rename(chemin_m3u8, format!("{}.bak", chemin_m3u8))?;
        fs::write(chemin_m3u8, lignes.join("\n"))?;
    }

    Ok(())
}

fn extraire(ligne: &str, attr: &str) -> String {
    let pattern = format!("{}=\"", attr);
    ligne.split_once(&pattern)
        .and_then(|(_, apres)| apres.split_once('"'))
        .map(|(valeur, _)| valeur.to_string())
        .unwrap_or_default()
}

fn remplacer(ligne: &str, attr: &str, valeur: &str) -> String {
    let ancienne = extraire(ligne, attr);
    if ancienne.is_empty() {
        return ligne.to_string();
    }
    ligne.replace(
        &format!("{}=\"{}\"", attr, ancienne),
        &format!("{}=\"{}\"", attr, valeur),
    )
}