use std::env;
// use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
mod category;
mod organizer;

fn main() {

    
    let args: Vec<String> = env::args().collect();
    let input = match args.get(1) {
        Some(p) => p,
        None => {
            println!("Please Provide a folder path to clean!");
            return;
        }
    };

    let output = match args.get(2) {
        Some(p) => p.to_string(),
        None => format!("{}/swipe_output", input),
    };

    let stats = organizer::file_transfer(input, &output);

    println!("===== SWIPE SUMMARY =====");
    println!("Moved  : {}", stats.moved);
    println!("Skipped: {}", stats.skipped);
    println!("Errors : {}", stats.errors);
  
}
