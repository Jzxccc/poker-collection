// JSON persistence: save/load decks to %LOCALAPPDATA%/poker-collection/poker_collection.json.

use std::fs;
use std::path::PathBuf;

use crate::deck::Deck;

fn app_data_dir() -> PathBuf {
    let local = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".into());
    PathBuf::from(local).join("poker-collection")
}

fn storage_path() -> PathBuf {
    app_data_dir().join("poker_collection.json")
}

fn ensure_dir() {
    let dir = app_data_dir();
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Warning: could not create data dir {:?}: {}", dir, e);
    }
}

pub fn load_decks() -> Vec<Deck> {
    let path = storage_path();
    match fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(decks) => decks,
            Err(e) => {
                eprintln!("Data file corrupted ({}). Starting fresh.", e);
                Vec::new()
            }
        },
        Err(_) => {
            Vec::new()
        }
    }
}

pub fn save_decks(decks: &[Deck]) {
    ensure_dir();
    let path = storage_path();
    let tmp = path.with_extension("json.tmp");
    let content = serde_json::to_string_pretty(decks).expect("serialization should succeed");
    fs::write(&tmp, &content).expect("write temp file should succeed");
    fs::rename(&tmp, &path).expect("atomic rename should succeed");
}
