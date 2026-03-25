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
        Ok(chemin) => {
            println!("Succès ! La playlist finale est ici : {}", chemin);
        }
        Err(e) => {
            eprintln!("Le processus a échoué lamentablement : {}", e);
            std::process::exit(1);
        }
    }

    println!("\n----- FIN -----");
}