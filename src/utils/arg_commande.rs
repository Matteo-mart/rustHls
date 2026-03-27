use std::env;

pub fn arg_commande() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <chemin_video>", args[0]);
        std::process::exit(1);
    }
    args[1].clone()
}