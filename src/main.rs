use std::env;
use std::fs;
use std::fs::read_dir;

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
    let output_base = &path;
    let mut moved = 0;
    let mut skipped = 0;
    let mut errors = 0;

    // Itertaing path item
    for i in files {
        match i {
            Ok(entry) => {
                let path = entry.path();

                if path.is_dir() {
                    skipped = skipped + 1;
                    continue;
                }
                
                let name = entry.file_name().to_string_lossy().into_owned();
                let extension = path.extension().and_then(|e| e.to_str());

                let category = match extension {
                    Some(
                        "rs" | "go" | "py" | "js" | "ts" | "cpp" | "c" | "java" | "html" | "css",
                    ) => "programming",
                    Some("json" | "toml" | "yaml" | "yml" | "xml" | "config" | "ini") => "configs",
                    Some("sh" | "bat" | "ps1") => "scripts",

                    Some("msi" | "exe" | "nsis" | "deb" | "rpm" | "dmg" | "iso") | Some("iss") => {
                        "installers"
                    }
                    Some("dll" | "sys" | "tmp") => "system_files",

                    Some("zip" | "rar" | "7z" | "tar" | "gz") => "archives",

                    Some("pdf") => "pdf_documents",
                    Some("doc" | "docx" | "rtf" | "odt") => "word_docs",
                    Some("xls" | "xlsx" | "csv") => "spreadsheets",
                    Some("ppt" | "pptx") => "presentations",
                    Some("txt") => "plain_text",

                    Some("png" | "jpg" | "jpeg" | "webp" | "svg" | "ico" | "gif") => "images",
                    Some("mp4" | "mkv" | "mov" | "avi") => "videos",
                    Some("mp3" | "wav" | "flac" | "aac") => "audios",

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
                moved = moved + 1;
                println!("\x1b[32m✓ Moved: {name} → {category}\x1b[0m");
            }
            Err(e) => {
                println!("Error: {e}");
                errors = errors + 1;
                continue;
            }
        }
    }
    println!("===== SUMMARY =====");
    println!("Moved: {moved}");
    println!("Skipped: {skipped}");
    println!("Error: {errors}");
    println!("===================");
}
