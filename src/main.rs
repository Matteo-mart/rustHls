mod utils;
mod execute;


fn main() {
    // println!("\nHello, world!");

    let _affichage_dossier = utils::affichage_dossier::affichage_dossier();
    let _delete = utils::delete::delete();
    let _create = utils::create::create();
    // let _convert_to_hls = execute::convert_to_hls::convert_to_hls("video/groovy-all-videos-and-all-audios.mp4");
    let _convert_to_hls = execute::convert_to_hls::convert_to_hls("video/test_comment_captions.mp4");

}
