// Libs
// use dotenv::dotenv;
use reqwest::{Client, Error};

use models::{catalog::CatalogResponse, webhook_message::WebhookMessage};

mod models;

// Functions
async fn notify(catalog: &CatalogResponse) -> Result<(), Error> {
    let webhook_url = std::env::var("WEBHOOK_URL").expect("Couldn\'t load the WEBHOOK_URL.");

    for game in catalog.data.catalog.search_store.elements.clone() {
        println!("Sending to the webhook informations about: {}", game.title);

        // Create the webhook message.
        let req_body = WebhookMessage::new(
            game.title,
            game.product_slug,
            game.key_images.first().unwrap(),
        );

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
    }

    Ok(())
}

// Main
#[tokio::main]
async fn main() {
    // dotenv().ok();

    // Get the current catalog.
    println!("Getting the current catalog...");

    let url = std::env::var("CATALOG_URL").expect("CATALOG_URL not found.");
    let catalog = Client::new()
        .get(url)
        .send()
        .await
        .expect("Couldn\'t connect to epicGames.")
        .json::<CatalogResponse>()
        .await
        .expect("Couldn\'t parse the response.");

    let mut cache = models::cache::Cache::new();
    let cache_catalog: Option<CatalogResponse> = cache.get_catalog();

    // Check if there's a new catalog.
    if cache_catalog.is_none() || catalog.data != cache_catalog.unwrap().data {
        println!("The cached catalog has updates.");
        let _ = match notify(&catalog).await {
            Ok(_) => cache.set_catalog(&catalog),
            Err(msg) => panic!("{}", msg),
        };
    } else {
        println!("The current catalog has any changes.");
    }
}

// fn main() {}
