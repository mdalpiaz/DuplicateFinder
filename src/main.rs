use walkdir::{WalkDir};
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose::STANDARD, Engine};
use std::{fs::File, io, collections::HashMap};

fn main() {
    let mut hashes: HashMap<String, Vec<String>> = HashMap::new();

    for file in WalkDir::new(".").into_iter().filter_map(|file| file.ok()).filter(|file| file.metadata().unwrap().is_file()) {
        let mut readable = match File::open(file.path()) {
            Ok(file) => file,
            Err(error) => panic!("File opening error: {:?}", error)
        };
        let mut hasher = Sha256::new();
        let _ = match io::copy(&mut readable, &mut hasher) {
            Ok(result) => result,
            Err(error) => panic!("io::copy error {:?}", error)
        };
        let hash = hasher.finalize();
        let b64 = STANDARD.encode(hash);
        hashes.entry(b64).or_insert(Vec::new()).push(file.path().display().to_string());
    }

    for (key, value) in hashes.iter().filter(|&(_, v)| v.len() > 1) {
        println!("{} -> {}", key, value.join(", "));
    }
}