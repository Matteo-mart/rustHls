use std::env;

/// récupère la vidéo mise dans la commande 
pub fn arg_commande() -> Result<String, String> {
    let args: Vec<String> = env::args()
        .collect();

    if args.len() < 2 {
        return Err("Erreur Commande".to_string());
    }
    
    Ok(args[1].clone())
}