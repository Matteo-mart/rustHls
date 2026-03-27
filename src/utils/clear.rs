use std::process::Command;

pub fn clear() {
    Command::new("sh")
        .arg("-c")
        .arg("clear")
        .status()
        .expect("\nErreur nettoyage terminal\n");
}