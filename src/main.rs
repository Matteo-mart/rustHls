mod utils;
mod execute;
mod convert;


fn main() {

    let video_source = "video/test_comment_captions.mp4";
    let directory_output = "tmp_result";

    utils::clear::clear();
    println!("\n----- LANCEMENT -----");

    execute::affichage::affichage_file_test("tmp_result");
    utils::delete_file_test::delete_file_test("tmp_result");
    utils::create_file_test::create_file_test("tmp_result");

    match convert::convert(video_source, directory_output) {
        Ok(chemin_playlist) => {
            println!("Succès ! Playlist générée : {}", chemin_playlist);

            let playlists = vec![chemin_playlist]; 
            
            println!("Génération de la master playlist...");
            if let Err(e) = execute::super_playlist::create_super_playlist(playlists, directory_output) {
                eprintln!("Erreur lors de la création de la super playlist : {}", e);
            } else {
                println!("Master playlist 'master.m3u8' créée avec succès dans {}", directory_output);
            }
        }
        Err(e) => {
            eprintln!("Le processus a échoué lamentablement : {}", e);
            std::process::exit(1);
        }
    }

    println!("\n----- FIN -----");
}