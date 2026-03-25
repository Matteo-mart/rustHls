// use std::collections::HashSet;
// use crate::utils::struct_types::Variant;

// pub fn create_super_playlist(playlists: Vec<String>, directory_out: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut unique_audios: HashSet<String> = HashSet::new();
//     let mut unique_videos: HashMap<String, Variant> = HashMap::new();

//     for path in playlist {
//         if let Err(e) = process_playlist(&path, &mut unique_audiosn &mut unique_videos) {
//             eprintln!("Erreur sur {} : {}", path, e),
//             continue;
//         }
//     }

//     write_playlist(directory_out, unique_audios, unique_videos)
// }

// fn 