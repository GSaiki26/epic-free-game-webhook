// Libs
use chrono::{DateTime, FixedOffset, Utc};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use super::catalog::StoreGame;

// Structs
pub struct DiscordWebhook;

// Implementations
impl DiscordWebhook {
    /**
     * A method to notify the webhook about a new game.
     */
    pub async fn send(embed: &DiscordWebhookMessage) -> Result<(), Error> {
        let webhook_url = std::env::var("WEBHOOK_URL").expect("Couldn\'t load the WEBHOOK_URL.");

        println!("Sending to the webhook...");

        // Send the webhook message. req_body is a json.
        let res = Client::new()
            .post(webhook_url.clone())
            .json(&embed)
            .send()
            .await;

        match res {
            Ok(_) => {
                println!("Webhook message sent.");
                Ok(())
            }
            Err(msg) => Err(msg),
        }
    }
}

// Serialization messages
#[derive(Serialize, Deserialize)]
pub struct DiscordWebhookMessage {
    pub embeds: Vec<DiscordEmbedMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordEmbedMessage {
    pub title: String,
    pub r#type: String,
    pub description: String,
    pub url: String,
    pub image: DiscordEmbedImage,
    pub footer: DiscordEmbedFooter,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordEmbedImage {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordEmbedFooter {
    pub text: String,
}

// Implementations
impl DiscordWebhookMessage {
    pub fn new(game: &StoreGame) -> Self {
        // Check if the game is active.
        let (offer_start, _) = Self::get_offer_dates(game);

        let mut description = format!("**Status: Available**\n\n{}", &game.description);
        if offer_start > Utc::now() {
            description = format!("**Status: Soon**\n\n{}", &game.description);
        }

        Self {
            embeds: vec![DiscordEmbedMessage {
                title: format!("Epic Games Store - {}", game.title),
                r#type: String::from("rich"),
                description,
                url: format!(
                    "https://store.epicgames.com/en-US/p/{}",
                    game.product_slug.as_ref().unwrap()
                ),
                image: DiscordEmbedImage {
                    url: game.key_images.first().unwrap().url.clone(),
                },
                footer: DiscordEmbedFooter {
                    text: Self::get_footer(game),
                },
            }],
        }
    }

    /**
     * A method to get the footer from the store game.
     * The footer contains the available period of the game.
     */
    fn get_footer(game: &StoreGame) -> String {
        // Parse the date.
        let (offer_start, offer_end) = Self::get_offer_dates(game);

        format!(
            "{} - {}",
            offer_start.format("%Y/%m/%d %H:%M"),
            offer_end.format("%Y/%m/%d %H:%M")
        )
    }

    /**
     * A method to get the dates from the offer.
     */
    fn get_offer_dates(game: &StoreGame) -> (DateTime<FixedOffset>, DateTime<FixedOffset>) {
        // Get the game promotion time.
        let offer = game
            .promotions
            .promotional_offers
            .first()
            .expect("Couldn\'t get the promotion time.")
            .promotional_offers
            .first()
            .unwrap();
        (
            DateTime::parse_from_rfc3339(&offer.start_date).unwrap(),
            DateTime::parse_from_rfc3339(&offer.end_date).unwrap(),
        )
    }
}
