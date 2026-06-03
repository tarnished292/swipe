use std::env;
use std::fs::read_dir;

use crate::category::get_category;
use crate::organizer::file_transfer;
mod category;
mod organizer;

fn main() {
    // Getting the path
    let args: Vec<String> = env::args().collect();
    let input_path = match args.get(1) {
        Some(p) => p,
        None => {
            println!("Please Provide a folder path to clean!");
            return;
        }
    };

    // Validating the path
    let files = match read_dir(input_path) {
        Ok(files) => files,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };

    // This is the locationnnn to store the organize file -> folder stuff
    let output_base = match args.get(2) {
        Some(p) => p.to_string(),
        None => format!("{}/swipe_output", input_path),
    };

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

                let category = get_category(extension);

                let target_folder = format!("{output_base}/{category}");

                file_transfer(&target_folder, &name, path);
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
