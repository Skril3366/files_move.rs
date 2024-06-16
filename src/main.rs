use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn find_files_matching_regex(dir: &Path, regex: &Regex) -> Vec<PathBuf> {
    let mut matching_files = Vec::new();
    let entries = fs::read_dir(dir).unwrap_or_else(|err| {
        eprintln!("Error reading directory: {}", err);
        std::process::exit(1);
    });

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            matching_files.append(&mut find_files_matching_regex(&path, regex));
        } else if regex.is_match(&path.to_string_lossy()) {
            matching_files.push(path);
        }
    }

    matching_files
}

fn create_dir_recursively(dir: &Path) -> Result<(), std::io::Error> {
    if !dir.exists() {
        fs::create_dir_all(dir)
    } else {
        Ok(())
    }
}

fn move_files_to_directory(files: Vec<PathBuf>, dest_dir: &Path) {
    for file in files {
        let dest_path = dest_dir.join(file.file_name().unwrap());
        match fs::rename(&file, &dest_path) {
            Ok(_) => println!("Moved file: {} to {}", file.display(), dest_path.display()),
            Err(err) => eprintln!("Error moving file: {}", err),
        }
    }
}

fn delete_empty_dirs_recursively(dir: &Path) -> Result<(), std::io::Error> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            delete_empty_dirs_recursively(&path)?;
        }
    }

    let mut entries = fs::read_dir(dir)?;
    if entries.next().is_none() && dir != Path::new(".") {
        fs::remove_dir(dir)?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <regex_pattern> <destination_directory>", args[0]);
        std::process::exit(1);
    }

    let regex_pattern = &args[1];
    let regex = Regex::new(regex_pattern).unwrap_or_else(|err| {
        eprintln!("Invalid regular expression: {}", err);
        std::process::exit(1);
    });

    let current_dir = env::current_dir().unwrap();
    let matching_files = find_files_matching_regex(&current_dir, &regex);
    let dest_dir_path = &args[2];
    let dest_dir = Path::new(dest_dir_path);
    let absolute_dest_dir = current_dir.join(dest_dir);

    create_dir_recursively(&absolute_dest_dir).unwrap_or_else(|err| {
        eprintln!("Error creating directory: {}", err);
        std::process::exit(1);
    });

    move_files_to_directory(matching_files, &absolute_dest_dir);

    delete_empty_dirs_recursively(&current_dir).unwrap_or_else(|err| {
        eprintln!("Error deleting empty directories: {}", err);
        std::process::exit(1);
    })
}
