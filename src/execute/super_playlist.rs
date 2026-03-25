use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};
use crate::utils::struct_types::Variant;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_GROUP_ID: Regex = Regex::new(r#"GROUP-ID="([^"]*)""#).unwrap();
    static ref RE_AUDIO_REF: Regex = Regex::new(r#"AUDIO="[^"]*""#).unwrap();
    static ref RE_BANDWIDTH: Regex = Regex::new(r"BANDWIDTH=(\d+)").unwrap();
    static ref RE_RESOLUTION: Regex = Regex::new(r"RESOLUTION=([\dx]+)").unwrap();
    static ref RE_CODECS: Regex = Regex::new(r#"CODECS="([^"]*)""#).unwrap();
}

pub fn create_super_playlist(playlists: Vec<String>, directory_out: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut unique_audios: HashSet<String> = HashSet::new();
    let mut unique_videos: HashMap<String, Variant> = HashMap::new();

    for path_str in playlists {
        let path = Path::new(&path_str);
        if let Err(e) = process_playlist(path, &mut unique_audios, &mut unique_videos) {
            eprintln!("Error processing {:?}: {}", path, e);
            continue;
        }
    }

    write_playlist(directory_out, unique_audios, unique_videos)
}

fn process_playlist(
    path: &Path,
    audios: &mut HashSet<String>,
    videos: &mut HashMap<String, Variant>,
) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|l| l.trim().to_string())
        .collect();

    let group_id = extract_group_id(&lines);

    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];

        if line.starts_with("#EXT-X-MEDIA:TYPE=AUDIO") {
            audios.insert(line.clone());
        }
        
        if line.starts_with("#EXT-X-STREAM-INF") && i + 1 < lines.len() {
            if !line.contains("RESOLUTION=") {
                i += 1;
                continue;
            }

            let mut meta = line.clone();
            if !group_id.is_empty() {
                meta = RE_AUDIO_REF.replace_all(line, format!(r#"AUDIO="{}""#, group_id).as_str()).to_string();
            }
            
            let url = lines[i + 1].clone();

            let bandwidth = RE_BANDWIDTH.captures(&meta)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse::<u64>().ok())
                .unwrap_or(0);

            let resolution = RE_RESOLUTION.captures(&meta)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let codecs = RE_CODECS.captures(&meta)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            videos.entry(meta.clone()).or_insert(Variant {
                bandwidth,
                resolution,
                codecs,
                uri: url.clone(),
            });

            i += 1;
        }
        i += 1;
    }
    Ok(())
}

fn extract_group_id(lines: &[String]) -> String {
    for line in lines {
        if line.starts_with("#EXT-X-MEDIA:TYPE=AUDIO") {
            if let Some(caps) = RE_GROUP_ID.captures(line) {
                return caps[1].to_string();
            }
        }
    }
    String::new()
}

fn write_playlist(
    directory_out: &str, 
    audios: HashSet<String>, 
    videos: HashMap<String, Variant>
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(directory_out).join("master.m3u8");
    let mut file = File::create(path)?;
    
    writeln!(file, "#EXTM3U")?;

    for audio_line in audios {
        writeln!(file, "{}", audio_line)?;
    }

    for (meta_original, variant) in videos {
        writeln!(file, "{}", meta_original)?;
        writeln!(file, "{}", variant.uri)?;
    }
    
    Ok(())
}