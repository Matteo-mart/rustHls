use std::env;
mod utils;
mod execute;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        std::process::exit(1);
    }

    let chemin_video = &args[2];
    // println!("Le chemin est {}", chemin_video);

    // let _affichage_dossier = utils::affichage_dossier::affichage_dossier();
    let _delete = utils::delete::delete();
    let _create = utils::create::create();
    let _convert_to_hls = execute::convert_to_hls::convert_to_hls(chemin_video);
    // let _convert_to_hls = execute::convert_to_hls::convert_to_hls("video/test_comment_captions.mp4");

}
