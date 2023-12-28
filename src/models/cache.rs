// Libs
use std::fs::File;
use std::io::Write;

use super::catalog::StoreGame;

// Structs
#[derive(Debug)]
pub struct Cache {
    filepath: String,
}

// Implementations
impl Cache {
    pub fn new() -> Self {
        // get the filepath and check if the file exists.
        let filepath = std::env::var("CACHE_PATH").expect("CACHE_PATH not found.");

        // Create the file if it doesn't exist.
        if !std::path::Path::new(&filepath).exists() {
            println!("Creating the cache file...");
            let mut cache_file = File::create(&filepath).expect("Couldn\'t create the cache file.");
            cache_file
                .write_all(b"[]")
                .expect("Couldn\'t write to the cache file.");
            cache_file.flush().expect("Couldn\'t flush the cache file.");
        }

        Self { filepath }
    }

    /**
     * A method to get a game from the cache.
     */
    pub fn get_game(&self, game_id: &str) -> Option<StoreGame> {
        println!("Getting from the cache the game#{}...", game_id);

        // Read the entire cache.
        let cache_file = File::open(&self.filepath).expect("Couldn\'t read the cache.");
        let cached_games: Vec<StoreGame> =
            serde_json::from_reader(cache_file).expect("Couldn\'t parse the cache.");

        // Return the game if it exists.
        cached_games.into_iter().find(|game| game.id == game_id)
    }

    /**
     * A method to get a game from the cache.
     */
    pub fn get_all_games(&self) -> Vec<StoreGame> {
        println!("Getting all games from the cache...");

        // Read the entire cache.
        let cache_file = File::open(&self.filepath).expect("Couldn\'t read the cache.");
        serde_json::from_reader(cache_file).expect("Couldn\'t parse the cache.")
    }

    /**
     * A method to add a game to the cache.
     */
    pub fn add_game(&self, game: &StoreGame) {
        println!("Adding to the cache the game#{}...", game.id);

        // Read the entire cache.
        let cache_file = File::open(&self.filepath).expect("Couldn\'t read the cache.");
        let mut cached_games: Vec<StoreGame> =
            serde_json::from_reader(cache_file).expect("Couldn\'t parse the cache.");

        cached_games.push(game.clone());

        // Write the cache back to the file.
        let mut cache_file = File::create(&self.filepath).expect("Couldn\'t read the cache.");
        cache_file
            .set_len(0)
            .expect("Couldn\'t clear the cache file.");
        serde_json::to_writer(&cache_file, &cached_games)
            .expect("Couldn\'t write to the cache file.");

        // Flush the file.
        cache_file.flush().expect("Couldn\'t flush the cache file.");
    }

    /**
     * A method to remove a game from the cache.
     */
    pub fn remove_game(&self, game_id: &str) {
        println!("Removing from the cache the game#{}...", game_id);

        // Read the entire cache.
        let cache_file = File::open(&self.filepath).expect("Couldn\'t read the cache.");
        let mut cached_games: Vec<StoreGame> =
            serde_json::from_reader(cache_file).expect("Couldn\'t parse the cache.");

        // Remove the game from the cache.
        cached_games.retain(|game| game.id != game_id);

        // Write the cache back to the file.
        let mut cache_file = File::create(&self.filepath).expect("Couldn\'t write the cache.");
        cache_file
            .set_len(0)
            .expect("Couldn\'t clear the cache file.");

        // Write the cache back to the file.
        match serde_json::to_writer(&cache_file, &cached_games) {
            Ok(_) => cache_file.flush().expect("Couldn\'t flush the cache file."),
            Err(_) => println!("Couldn\'t remove the game from the cache."),
        }
    }
}
