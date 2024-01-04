// Libs
use models::catalog::get_catalog;

use models::{
    cache::Cache,
    catalog::StoreGame,
    discord_webhook::{DiscordWebhook, DiscordWebhookMessage},
};

mod models;

// Functions
/**
 * A method to check new games.
*/

async fn check_new_games(cache: &mut Cache, games: &Vec<StoreGame>) {
    println!("Checking new games...\n");

    for game in games {
        println!("Checking \"{}\" from catalog...", game.title);

        // Check if the game is in the cache.
        let cached_game = cache.get_game(&game.id);
        if let Some(cached_game) = cached_game {
            // Check if the title and the expiry_date is the same.
            if game == &cached_game {
                println!("\"{}\" is already in the cache.\n", game.title);
                continue;
            }
        }

        // Notify the webhook.
        let embed = DiscordWebhookMessage::new(game);
        match DiscordWebhook::send(&embed).await {
            Ok(_) => cache.add_game(game),
            Err(msg) => println!("Couldn't notify the webhook: {}", msg),
        };
    }
}

/**
 * A method to check the games that are stored in the cache.
*/
fn remove_old_games(cache: &mut Cache, games: Vec<StoreGame>) {
    println!("Checking old games...");

    let cached_games = cache.get_all_games();
    for cached_game in cached_games {
        println!("Checking \"{}\" from cache...", cached_game.title);

        // Check if the game is in the catalog.
        if games.iter().any(|game| game == &cached_game) {
            println!("\"{}\" is still in the catalog.\n", cached_game.title);
            continue;
        }

        // If the game if not in the catalog, remove it from the cache.
        println!("The game is not in the catalog anymore.");
        cache.remove_game(&cached_game.id);
    }
}

// Main
#[tokio::main]
async fn main() {
    let catalog = get_catalog().await;
    let mut cache = models::cache::Cache::new();
    let games = catalog.data.catalog.search_store.elements.clone();

    check_new_games(&mut cache, &games).await;
    remove_old_games(&mut cache, games);
}

// fn main() {}
