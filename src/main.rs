use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use walkdir::WalkDir;
use blake3;

fn compute_file_hash(path: PathBuf) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);
    let hash = hasher.finalize();
    Ok(hash.to_hex().to_string())
}

fn walk_dir(dir: PathBuf) -> io::Result<HashMap<String, PathBuf>> {
    let mut hashes = HashMap::new();
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let hash = compute_file_hash(entry.path().to_path_buf())?;
            hashes.insert(hash, entry.into_path());
        }
    }
    Ok(hashes)
}

fn compare_and_print(dir1: HashMap<String, PathBuf>, dir2: HashMap<String, PathBuf>) {
    for (hash, path) in &dir1 {
        match dir2.get(hash) {
            Some(path2) if path != path2 => {
                println!("Same hash but different paths: {:?} and {:?}", path, path2);
            },
            None => {
                println!("Unique in first directory: {:?}", path);
            },
            _ => {}
        }
    }

    for (hash, path) in &dir2 {
        if !dir1.contains_key(hash) {
            println!("Unique in second directory: {:?}", path);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <directory1> <directory2>", args[0]);
        std::process::exit(1);
    }

    let dir1 = PathBuf::from(&args[1]);
    let dir2 = PathBuf::from(&args[2]);

    let dir1_hashes = walk_dir(dir1).expect("Failed to walk through first directory");
    let dir2_hashes = walk_dir(dir2).expect("Failed to walk through second directory");

    compare_and_print(dir1_hashes, dir2_hashes);
}
