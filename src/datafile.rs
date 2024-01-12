// Libs
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::serialization::data::{DataFile, DataFileWebhook};
use crate::serialization::epic_catalog::StoreGame;

// Structs
/// A struct to manage the `data.json` file.
/// It contains method to save and remove games from the `data`.
///
pub struct Datafile {
    filepath: String,
}

// Implementations
impl Datafile {
    pub fn new() -> io::Result<Self> {
        // get the filepath and check if the file exists.
        let data = Self {
            filepath: match std::env::var("DATA_PATH") {
                Ok(value) => value,
                Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "DATA_PATH not found.")),
            },
        };

        // Return the file if it exists.
        if Path::new(&data.filepath).exists() {
            return Ok(data);
        }

        // Create the file if it doesn't exist.
        println!("Creating the cache file...");
        data.set_file(DataFile::default())?;

        Ok(data)
    }

    /**
     * A method to get a game from the `data.json` file.
     */
    pub fn get_game(&self, game_id: &str) -> io::Result<Option<StoreGame>> {
        println!("Checking if game is saved...");

        // Read the entire cache.
        let content_file = self.get_file()?;

        // Return the game if it exists.
        Ok(content_file
            .games
            .into_iter()
            .find(|game| game.id == game_id))
    }

    /**
     * A method to get a game from the `data.json` file.
     */
    pub fn get_all_games(&self) -> io::Result<Vec<StoreGame>> {
        // Read the entire cache.
        let file_content = self.get_file()?;
        Ok(file_content.games)
    }

    /**
     * A method to get a game from the `data.json` file.
     */
    pub fn get_all_webhooks(&self) -> io::Result<Vec<DataFileWebhook>> {
        // Read the entire cache.
        let file_content = self.get_file()?;
        Ok(file_content.webhooks)
    }

    /**
     * A method to add a game to the `data.json` file.
     */
    pub fn add_games(&self, games: &[StoreGame]) -> io::Result<()> {
        println!("Saving new games...");

        // Read the entire cache.
        let mut file_content = self.get_file()?;

        // Add the games to the cache.
        for game in games {
            file_content
                .games
                .retain(|cached_game| cached_game.id != game.id);

            // Remove the game from the cache if it already exists, and add the new one.
            file_content.games.push(game.clone());
        }

        // Write the cache back to the file.
        self.set_file(file_content)
    }

    /**
     * A method to check the games that are stored in the `data.json` file.
     */
    pub fn remove_old_games(&mut self, current_games: Vec<StoreGame>) -> io::Result<()> {
        println!("Removing old games...\n");

        let cached_games = self.get_all_games()?;
        for cached_game in cached_games {
            println!("Checking if \"{}\" is in the catalog...", cached_game.title);

            // Check if the game is in the catalog.
            if current_games.iter().any(|game| game == &cached_game) {
                println!("\"{}\" is still in the catalog.\n", cached_game.title);
                continue;
            }

            // If the game if not in the catalog, remove it from the cache.
            println!("The game is not in the catalog anymore. Removing it...");
            self.remove_game(&cached_game.id)?;
        }

        Ok(())
    }

    /**
     * A method to remove a game from the `data.json` file.
     */
    fn remove_game(&self, game_id: &str) -> io::Result<()> {
        println!("Removing the game...");

        // Read the entire data.
        let mut file_content = self.get_file()?;

        // Remove the game from the data.
        file_content.games.retain(|game| game.id != game_id);

        // Write the data back to the file.
        self.set_file(file_content)
    }

    /**
     * A method to get the file's content.
     */
    fn get_file(&self) -> io::Result<DataFile> {
        let file = File::open(&self.filepath)?;
        match serde_json::from_reader(file) {
            Ok(content) => Ok(content),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Couldn\'t read the file.",
            )),
        }
    }

    /**
     * A method to overwrite the content from the `data.json` with some content.
     */
    fn set_file(&self, content: DataFile) -> io::Result<()> {
        // Open as Writer and clear the file.
        let mut cache_file = File::create(&self.filepath)?;
        cache_file.set_len(0)?;

        // Write the cache back to the file.
        match serde_json::to_writer_pretty(&cache_file, &content) {
            Ok(_) => cache_file.flush(),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Couldn\'t write to the cache file.",
            )),
        }
    }
}
