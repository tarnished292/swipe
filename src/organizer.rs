use crate::category::get_category;
use std::fs::{self, read_dir};

use indicatif::{ProgressBar, ProgressStyle};

pub struct TransferStats {
    pub moved: u32,
    pub skipped: u32,
    pub errors: u32,
}

pub fn file_transfer(input_path: &str, output_base: &str) -> TransferStats {
    // Validating the path
    let files: Vec<_> = match read_dir(input_path) {
        Ok(files) => files.collect(),
        Err(_) => {
            return TransferStats {
                moved: 0,
                skipped: 0,
                errors: 1,
            };
        }
    };

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "\n{spinner:.cyan.bold} {msg}\n [{bar:45.green/dim}] {pos}/{len} ({percent}%) ⏱ {elapsed}"
        )
        .unwrap()
        .tick_strings(&["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"])
        .progress_chars("█▓░"),
    );
    pb.set_message("Sorting files...");

    let mut moved = 0;
    let mut skipped = 0;
    let mut errors = 0;

    for entry in files {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_dir() {
                    skipped += 1;
                    pb.inc(1);
                    continue;
                }

                let name = entry.file_name().to_string_lossy().into_owned();
                let ext = path.extension().and_then(|e| e.to_str());

                let category = get_category(ext);

                let target_folder = format!("{output_base}/{category}");

                if let Err(e) = fs::create_dir_all(&target_folder) {
                    println!("Failed to create folder: {}", e);
                    errors += 1;
                    pb.inc(1);
                    continue;
                }

                let dest = format!("{target_folder}/{name}");

                if let Err(e) = fs::rename(&path, &dest) {
                    pb.println(format!("Error moving {name}: {e}"));
                    pb.inc(1);
                    continue;
                }

                moved += 1;
                pb.println(format!("\x1b[32m✓ Moved: {name} → {category}\x1b[0m"));
                pb.inc(1);
            }
            Err(e) => {
                pb.println(format!("Error reading entry: {e}"));
                errors += 1;
                pb.inc(1);
                continue;
            }
        }
    }

    pb.finish_with_message("Transfering Complete");

    TransferStats {
        moved,
        skipped,
        errors,
    }
}
