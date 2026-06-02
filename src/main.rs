use std::env;
use std::fs;
use std::fs::read_dir;
use std::fs::rename;

fn main() {
    // Getting the path
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(p) => p,
        None => {
            println!("Please Provide a folder path to clean!");
            return;
        }
    };

    // Validating the path
    let files = match read_dir(path) {
        Ok(files) => files,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };

    // This is the locationnnn to store the organize file folder stuff
    let output_base = "D:/rustBroom_test/";

    // Itertaing path item
    for i in files {
        match i {
            Ok(entry) => {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().into_owned();
                let extension = path.extension().and_then(|e| e.to_str());

                let category = match extension {
                    Some("rs") => "code",
                    Some("pdf") | Some("txt") | Some("csv") => "documents",
                    Some("json") => "sheet",
                    Some("zip") => "zip",
                    Some("msi") | Some("exe") => "installer",
                    Some("toml") | Some("lock") => "build",
                    Some("png") | Some("jpg") => "images",
                    Some("mp4") => "videos",
                    Some("mp3") | Some("wav") => "audios",
                    Some(other) => other,
                    None => "unknown",
                };

                let target_folder = format!("{output_base}/{category}");
                if let Err(e) = fs::create_dir_all(&target_folder) {
                    println!("Failed to create folder: {}", e);
                    continue;
                }

                let dest = format!("{target_folder}/{name}");
                if let Err(e) = fs::rename(&path, &dest) {
                    println!("Error: {e}");
                    continue;
                }

                println!("File name {name}, Category: {:?}", category);
            }
            Err(e) => {
                println!("Error: {e}");
                return;
            }
        }
    }
}
