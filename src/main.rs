use std::env;
mod utils;
mod execute;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        std::process::exit(1);
    }

    let chemin_video = &args[2];
    let file_tmp_result = "tmp_result";
    // println!("Le chemin est {}", chemin_video);

    let _delete = utils::delete::delete(file_tmp_result);
    let _create = utils::create::create(file_tmp_result);
    let _convert_to_hls = execute::convert_to_hls::convert_to_hls(chemin_video);

    println!("\n----- FIN -----")

}
