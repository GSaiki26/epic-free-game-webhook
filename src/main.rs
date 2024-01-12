// Libs
use std::process::exit;

use reqwest::Client;
use tokio::spawn;

use datafile::Datafile;
use serialization::epic_catalog::{CatalogResponse, StoreGame};
use webhooks::webhook::Webhook;

mod datafile;
mod serialization;
mod webhooks;

// Functions
/**
 * A method to check new games.
*/
fn check_new_games(data: &Datafile, games: &[StoreGame]) -> Vec<StoreGame> {
    println!("Checking new games...\n");

    let mut new_games = games.to_vec();

    // Just select the new games.
    new_games.retain(|new_game| {
        println!("Checking if \"{}\" is saved...", new_game.title);

        match data.get_game(&new_game.id) {
            Err(err) => {
                eprintln!("Couldn\'t check if the game is saved: {}", err);
                exit(1)
            }
            Ok(cached_game) => match cached_game {
                None => {
                    println!("\"{}\" is new.\n", new_game.title);
                    true
                }
                Some(cache_game) if new_game == &cache_game => {
                    println!("\"{}\" is already in the cache.\n", cache_game.title);
                    false
                }
                Some(_) => {
                    println!(
                        "\"{}\" is already in the cache, but it's different.\n",
                        new_game.title
                    );
                    true
                }
            },
        }
    });

    new_games
}

/**
 * A method to get the current catalog from EpicGames.
*/
async fn get_catalog() -> Result<CatalogResponse, &'static str> {
    println!("Getting the current online catalog...");

    // Get the current catalog.
    let url = std::env::var("CATALOG_URL").expect("CATALOG_URL not found.");
    let response = Client::new().get(url).send().await;

    match response {
        Err(_) => Err("Couldn\'t connect to epicGames."),
        Ok(res) => match res.json::<CatalogResponse>().await {
            Err(_) => Err("Couldn\'t parse the request body."),
            Ok(body) => Ok(body),
        },
    }
}

// Main
#[tokio::main]
async fn main() {
    // Get the epic catalog.
    let catalog = match get_catalog().await {
        Ok(body) => body,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    println!("Done!\n");

    // Get the data file.
    let mut data = match Datafile::new() {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    // Get the saved games and check the new ones.
    let games = catalog.data.catalog.search_store.elements.clone();
    let new_games = check_new_games(&data, &games);
    if new_games.is_empty() {
        println!("No new game has been added to the catalog.");
        exit(0);
    };
    println!("Done!\n");

    // Remove the old games from the datafile.
    if let Err(err) = data.remove_old_games(games) {
        eprintln!("{}", err);
        exit(1);
    };
    println!("Done!\n");

    // Add the new ones to the datafile.
    if let Err(err) = data.add_games(&new_games) {
        eprintln!("{}", err);
        exit(1);
    }
    println!("Done!\n");

    // Convert the data file webhooks to webhooks (Webhooks that implements the trait Webhook).
    let webhooks = match data.get_all_webhooks() {
        Ok(webhooks) => webhooks,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    println!("{} webhooks found!\n", webhooks.len());
    if webhooks.is_empty() {
        println!("Nothing to do!");
        exit(0);
    }

    let webhooks = webhooks.iter().map(|webhook| webhook.clone().to_webhook());

    // Send the new games to the webhooks.
    println!("Sending the new games to all provided webhooks...");
    let mut handlers = Vec::new();
    for webhook in webhooks {
        let new_games = new_games.clone();
        let handler = spawn(async move {
            if webhook.send_games(&new_games).await.is_err() {
                eprintln!("Couldn\'t send the games to {}.", webhook.get_title());
            }
        });
        handlers.push(handler);
    }

    // Wait for all the handlers to finish.
    for handler in handlers {
        if let Err(err) = handler.await {
            eprintln!("A handler has panicked. {}", err);
        }
    }

    println!("\nAll webhooks has been executed!");
}

// fn main() {}
