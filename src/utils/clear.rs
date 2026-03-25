use std::process::Command;

/*
    Nettoie le terminal
*/
pub fn clear() {

    Command::new("clear")
    .status()
    .expect("Erreur lors du clear du terminal");
}