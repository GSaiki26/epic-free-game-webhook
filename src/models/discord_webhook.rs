// Libs
use chrono::DateTime;
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
        if game.product_slug.is_none() || game.product_slug.as_ref().unwrap() == "[]" {
            return DiscordWebhookMessage {
                embeds: vec![DiscordEmbedMessage {
                    title: format!("Free Game on Epic Store - {}", game.title),
                    r#type: String::from("rich"),
                    description: format!("**Status: Soon**\n\n{}", &game.description),
                    url: String::from("https://www.epicgames.com/store/en-US/free-games"),
                    image: DiscordEmbedImage {
                        url: game.key_images.first().unwrap().url.clone(),
                    },
                    footer: DiscordEmbedFooter {
                        text: String::from("Soon"),
                    },
                }],
            };
        }

        let footer_text = Self::get_footer(game);

        Self {
            embeds: vec![DiscordEmbedMessage {
                title: format!("Epic Games Store - {}", game.title),
                r#type: String::from("rich"),
                description: format!("**Status: Available**\n\n{}", &game.description),
                url: format!(
                    "https://store.epicgames.com/en-US/p/{}",
                    game.product_slug.as_ref().unwrap()
                ),
                image: DiscordEmbedImage {
                    url: game.key_images.first().unwrap().url.clone(),
                },
                footer: DiscordEmbedFooter { text: footer_text },
            }],
        }
    }

    /**
     * A method to get the footer from the store game.
     * The footer contains the available period of the game.
     */
    fn get_footer(game: &StoreGame) -> String {
        // Parse the date.
        let dt_init = DateTime::parse_from_rfc3339(&game.effective_date).unwrap();
        let dt_init = dt_init.with_timezone(&chrono::Local);
        let mut footer_text = format!("{}", dt_init.format("%Y/%m/%d %H:%M"));

        if let Some(dt) = game.expiry_date.clone() {
            let dt = DateTime::parse_from_rfc3339(&dt).unwrap();
            let dt = dt.with_timezone(&chrono::Local);
            footer_text = format!("{} - {}", footer_text, dt.format("%Y/%m/%d %H:%M"));
        }

        footer_text
    }
}
