use std::fs;
use std::io;

pub fn create(file_tmp_result: &str) -> io::Result<()> {
    let result = fs::create_dir(file_tmp_result);
    
    if let Err(ref e) = result {
        eprintln!("\nErreur création dossier '{}': {}\n", file_tmp_result, e);
    }
    
    result
}