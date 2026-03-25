use std::path::Path;

/*
    Prend le nom du fichier et retire les extentions
*/
pub fn file_name_without_extension(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}