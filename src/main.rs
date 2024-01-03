// Libs
use reqwest::{Client, Error};

use models::{
    cache::Cache,
    catalog::{CatalogResponse, StoreGame},
    webhook_message::WebhookMessage,
};

mod models;

// Functions
/**
 * A method to check new games.
*/
async fn check_new_games(cache: &mut Cache, games: &Vec<StoreGame>) {
    println!("\nChecking new games...");

    for game in games {
        println!("Checking \"{}\" from catalog...", game.title);

        // Check if the game is in the cache.
        let game_in_cache = cache.get_game(&game.id);
        if game_in_cache.is_some() && game_in_cache.unwrap().title == game.title {
            println!("\"{}\" is already in the cache.", game.title);
            continue;
        }

        // Notify the webhook.
        match notify_game(game).await {
            Ok(_) => cache.add_game(game),
            Err(msg) => println!("Couldn\'t notify the webhook: {}", msg),
        }
    }
}

/**
 * A method to check the games that are stored in the cache.
*/
fn remove_old_games(cache: &mut Cache, games: Vec<StoreGame>) {
    println!("\nChecking old games...");

    let cached_games = cache.get_all_games();
    for cached_game in cached_games {
        println!("Checking \"{}\" from cache...", cached_game.title);

        // Check if the game is in the catalog.
        if games.iter().any(|game| game.id == cached_game.id) {
            println!("\"{}\" is still in the catalog.", cached_game.title);
            continue;
        }

        // If the game if not in the catalog, remove it from the cache.
        println!(
            "\"{}\" is not in the catalog anymore. Removing...",
            cached_game.title
        );
        cache.remove_game(&cached_game.id);
    }
}

/**
 * A method to notify the webhook about a new game.
*/
async fn notify_game(game: &StoreGame) -> Result<(), Error> {
    let webhook_url = std::env::var("WEBHOOK_URL").expect("Couldn\'t load the WEBHOOK_URL.");

    println!("Sending to the webhook informations about: {}", game.title);

    // Create the webhook message.
    let req_body = WebhookMessage::new(game);

    // Send the webhook message. req_body is a json.
    let res = Client::new()
        .post(webhook_url.clone())
        .json(&req_body)
        .send()
        .await;

    match res {
        Ok(response) => {
            println!("{}", response.text().await?);
            println!("Webhook message sent.");
        }
        Err(msg) => return Err(msg),
    }

    Ok(())
}

/**
 * A method to get the current catalog from EpicGames.
*/
async fn get_catalog() -> CatalogResponse {
    println!("Getting the current catalog...");

    // Get the current catalog.
    let url = std::env::var("CATALOG_URL").expect("CATALOG_URL not found.");
    Client::new()
        .get(url)
        .send()
        .await
        .expect("Couldn\'t connect to epicGames.")
        .json::<CatalogResponse>()
        .await
        .expect("The format of the catalog is not valid. Please check the CATALOG_URL.")
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
