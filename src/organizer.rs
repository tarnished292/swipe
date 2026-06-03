use std::fs;

pub fn file_transfer(target_folder: &str, name: &str, path: std::path::PathBuf) {
    if let Err(e) = fs::create_dir_all(&target_folder) {
        println!("Failed to create folder: {}", e);
        return;
    }

    let dest = format!("{target_folder}/{name}");
    if let Err(e) = fs::rename(&path, &dest) {
        println!("Error: {e}");
        return;
    } 
}
