use std::collections::HashMap;
use std::env;
use std::io::{self};
use std::path::Path;
use jwalk::{Parallelism, WalkDir};
use blake3;

fn compute_file_hash(path: &Path) -> io::Result<String> {
    let mut hasher = blake3::Hasher::new();
    hasher.update_mmap(path)?;
    Ok(hasher.finalize().to_hex().to_string())
}

fn walk_dir(dir: &Path) -> io::Result<HashMap<String, String>> {
    let mut hashes = HashMap::new();
    for entry in WalkDir::new(dir).parallelism(Parallelism::RayonNewPool(0)).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let hash = compute_file_hash(&path)?;
            let relative_path = path.strip_prefix(dir)
                .expect("Failed to get relative path")
                .to_str()
                .expect("Failed to convert path to string")
                .to_string();
            hashes.insert(hash, relative_path);
        }
    }
    Ok(hashes)
}

fn compare_and_print(dir1: &HashMap<String, String>, dir2: &HashMap<String, String>) {
    for (hash, rel_path) in dir1 {
        match dir2.get(hash) {
            Some(rel_path2) if rel_path != rel_path2 => {
                println!("Same hash but different relative paths: \n{:?} and {:?}", rel_path, rel_path2);
            },
            Some(_) => {}, // Same hash and same relative path
            None => {
                println!("Unique in first directory: {:?}", rel_path);
            },
        }
    }

    for (hash, _rel_path) in dir2 {
        match dir1.get(hash) {
            Some(_) => {}, // Same hash and same relative path
            None => {
                println!("Unique in second directory: {:?}", _rel_path);
            },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <directory1> <directory2>", args[0]);
        std::process::exit(1);
    }

    let dir1 = Path::new(&args[1]);
    let dir2 = Path::new(&args[2]);

    let dir1_hashes = walk_dir(dir1).expect("Failed to walk through first directory");
    let dir2_hashes = walk_dir(dir2).expect("Failed to walk through second directory");

    compare_and_print(&dir1_hashes, &dir2_hashes);
}
