use std::fs;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use crate::execute::ffprobe::ffprobe;


pub fn modifier_playlist(chemin_m3u8: &str, video_source: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let res = ffprobe(video_source).map_err(|e| e.to_string())?;
    let mut audio_map: HashMap<String, bool> = HashMap::new();

    for s in res.streams {
        if s.codec_type == "audio" {
            // Accès direct à la HashMap (plus besoin de as_ref ou and_then)
            let lang = s.tags
                .get("language")
                .cloned() // On clone le String trouvé dans l'Option
                .unwrap_or_else(|| "und".to_string());

            // Détection AD basée sur tes nouveaux champs à plat
            let is_ad = s.disposition_descriptions == 1; 

            let entry = audio_map.entry(lang).or_insert(false);
            if is_ad {
                *entry = true;
            }
        }
    }

    // Lecture du fichier M3U8
    let contenu = fs::read_to_string(chemin_m3u8)?;
    let mut nouvelles_lignes = Vec::new();

    // Traitement des lignes
    for ligne in contenu.lines() {
        let mut ligne_traitee = ligne.to_string();

        if ligne.starts_with("#EXT-X-MEDIA:TYPE=AUDIO") {
            let lang = extraire(ligne, "LANGUAGE");
            let mut nouveau_name = lang.clone();

            // Vérifier si la langue est marquée comme AD dans notre map
            if *audio_map.get(&lang).unwrap_or(&false) {
                nouveau_name = "AD".to_string();

                // Ajout des CHARACTERISTICS si absentes
                if !ligne.contains("CHARACTERISTICS") {
                    let tag = "CHARACTERISTICS=\"public.accessibility.describes-video\"";
                    // On insère le tag juste avant NAME pour respecter la structure habituelle
                    ligne_traitee = ligne_traitee.replacen("NAME=", &format!("{},NAME=", tag), 1);
                }
            }

            // Application du changement de NAME
            ligne_traitee = remplacer(&ligne_traitee, "NAME", &nouveau_name);
        }
        
        nouvelles_lignes.push(ligne_traitee);
    }

    // 4. Réécriture du fichier
    let mut flux_sortie = fs::File::create(chemin_m3u8)?;
    for l in nouvelles_lignes {
        writeln!(flux_sortie, "{}", l)?;
    }

    Ok(())
}

fn extraire(ligne: &str, attr: &str) -> String {
    let pattern = format!("{}=\"", attr);
    let p: Vec<&str> = ligne.split(&pattern).collect();

    if p.len() < 2 {
        return "".to_string();
    }

    p[1].split('"').next().unwrap_or("").to_string()
}

fn remplacer(ligne: &str, attr: &str, valeur: &str) -> String {
    let ancienne = extraire(ligne, attr);

    if ancienne.is_empty() {
        return ligne.to_string();
    }

    let cible = format!("{}=\"{}\"", attr, ancienne);
    let nouveau = format!("{}=\"{}\"", attr, valeur);

    ligne.replacen(&cible, &nouveau, 1)
}