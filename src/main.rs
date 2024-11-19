use sha256::try_digest;
use std::{
    collections::HashMap,
    fs,
    io::{stdin, stdout, Error, Write},
    path::{Path, PathBuf},
};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

fn calc_hash(path: &Path) -> Result<String, Error> {
    try_digest(path)
}

fn collect_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    files.extend(collect_files(&path)); // Llamada recursiva
                } else if path.is_file() {
                    files.push(path);
                }
            }
        } else {
            eprintln!("{} Failed to read directory '{}'", "[ERROR]".red(), dir.display());
        }
    }
    files
}

fn main() {
    let mut input = String::new();

    print!("Enter the path of the directory you want to analyze: ");
    stdout().flush().unwrap(); // Muestra la salida en tiempo real

    stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Elimina saltos de línea y espacios en blanco

    let dir_path = Path::new(input);
    if !dir_path.exists() {
        eprintln!("{} The specified path '{}' doesn't exist.", "[ERROR]".red(), input);
        return;
    }

    if !dir_path.is_dir() {
        eprintln!("{} The specified path '{}' is not a directory.", "[ERROR]".red(), input);
        return;
    }

    let files = collect_files(dir_path);
    if files.is_empty() {
        println!("No files found in the specified directory.");
        return;
    }

    // Barra de progreso
    let bar = ProgressBar::new(files.len() as u64);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} {prefix:.bold.dim} {pos}/{len} [{eta_precise}] {wide_bar:.green} {percent}%")
        .expect("Failed to set progress bar template");
    bar.set_prefix("Processing files...");
    bar.set_style(style.progress_chars("█▉░"));

    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for file in &files {
        match calc_hash(file) {
            Ok(hash) => {
                hash_map.entry(hash).or_default().push(file.to_path_buf());
            }
            Err(e) => eprintln!("{} Error calculating hash for '{:?}': {}", "[ERROR]".red(), file, e),
        }
        bar.inc(1); // Actualiza la barra de progreso
    }
    bar.finish_with_message("File processing completed.");

    // Busca duplicados y los muestra
    let mut dups_found = false;
    for (hash, paths) in &hash_map {
        if paths.len() > 1 {
            dups_found = true;
            println!("Duplicate files with hash '{}':", hash);
            for path in paths {
                println!(" - {}", path.display().to_string().green());
            }
        }
    }

    if !dups_found {
        println!("No duplicate files found.");
    }
}
