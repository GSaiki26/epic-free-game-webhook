// Libs
use std::io::{Seek, Write};

use super::catalog::CatalogResponse;

// Structs
#[derive(Debug)]
pub struct Cache {
    file: std::fs::File,
}

// Implementations
impl Cache {
    pub fn new() -> Self {
        let filepath = std::env::var("CACHE_PATH").expect("CACHE_PATH not found.");
        let filepath = std::path::Path::new(&filepath);
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filepath)
            .expect("Couldn\'t open the cache file.");

        // If the file is empty, write an empty json.
        if file.metadata().unwrap().len() == 0 {
            file.write_all(b"{}")
                .expect("Couldn\'t write to the cache file.");
            file.flush().expect("Couldn\'t flush the cache file.");
        }

        Self { file }
    }

    pub fn get_catalog(&self) -> Option<CatalogResponse> {
        println!("Getting the cached catalog...");

        // Read the entire cache.
        serde_json::from_reader(&self.file).ok()
    }

    pub fn set_catalog(&mut self, catalog: &CatalogResponse) -> Result<(), String> {
        println!("Setting the cached catalog...");

        // Write the entire file with the new catalog.
        self.file
            .seek(std::io::SeekFrom::Start(0))
            .expect("Couldn\'t seek the cache file.");

        let catalog = serde_json::to_string(&catalog).unwrap();
        self.file
            .write_all(catalog.as_bytes())
            .expect("Couldn\'t write to the cache file.");
        self.file.flush().expect("Couldn\'t flush the cache file.");

        Ok(())
    }
}
