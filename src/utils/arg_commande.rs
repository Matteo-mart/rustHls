use std::{env};

///commande terminal
pub fn arg_commande() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("\nMauvaise commande\n");
        std::process::exit(1);
    }
    args[1].clone()
}