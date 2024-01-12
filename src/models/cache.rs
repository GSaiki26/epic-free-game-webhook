// Libs
use std::fs::File;
use std::io::{self, Write};

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
        println!("Searching the game in the cache...");

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
        println!("Adding the game to the cache...");

        // Read the entire cache.
        let cache_file = File::open(&self.filepath).expect("Couldn\'t read the cache.");
        let mut cached_games: Vec<StoreGame> =
            serde_json::from_reader(cache_file).expect("Couldn\'t parse the cache.");

        // Remove the game from the cache if it already exists.
        cached_games.retain(|cached_game| cached_game.id != game.id);
        cached_games.push(game.clone());

        // Write the cache back to the file.
        match self.to_writer(&cached_games) {
            Ok(_) => println!("Game added to the cache.\n"),
            Err(_) => eprintln!("Couldn\'t add the game to the cache."),
        }
    }

    /**
     * A method to remove a game from the cache.
     */
    pub fn remove_game(&self, game_id: &str) {
        println!("Removing the game from the cache...");

        // Read the entire cache.
        let cache_file = File::open(&self.filepath).expect("Couldn\'t read the cache.");
        let mut cached_games: Vec<StoreGame> =
            serde_json::from_reader(cache_file).expect("Couldn\'t parse the cache.");

        // Remove the game from the cache.
        cached_games.retain(|game| game.id != game_id);

        // Write the cache back to the file.
        match self.to_writer(&cached_games) {
            Ok(_) => println!("Game removed from the cache.\n"),
            Err(_) => eprintln!("Couldn\'t remove the game from the cache."),
        }
    }

    fn to_writer(&self, games: &Vec<StoreGame>) -> io::Result<()> {
        // Open as Writer and clear the file.
        let mut cache_file = File::create(&self.filepath)?;
        cache_file.set_len(0)?;

        // Write the cache back to the file.
        match serde_json::to_writer(&cache_file, &games) {
            Ok(_) => cache_file.flush(),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Couldn\'t write to the cache file.",
            )),
        }
    }
}
