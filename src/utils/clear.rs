use std::process::Command;

///nettoie le terminal
pub fn clear() {

    Command::new("clear")
    .status()
    .expect("Erreur lors du clear du terminal");
}