// Libs
use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::epic_catalog::StoreGame;

// Serialization
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
impl From<&StoreGame> for DiscordEmbedMessage {
    fn from(game: &StoreGame) -> Self {
        // Get the game informations.
        let url = match game.product_slug.as_ref() {
            Some(product_slug) => format!("https://store.epicgames.com/en-US/p/{}", product_slug),
            None => String::from("https://www.epicgames.com/store/en-US/free-games"),
        };

        let mut description = format!("**Status: Soon**\n\n{}", &game.description);
        let mut footer = String::from("N/A");

        if let Some(offer_dts) = game.get_offer_datetimes() {
            let (offer_start, offer_end) = offer_dts;

            if Utc::now() >= offer_start {
                description = format!("**Status: Available**\n\n{}", &game.description);
            }
            footer = format!(
                "{} - {}",
                offer_start.format("%Y/%m/%d %H:%M"),
                offer_end.format("%Y/%m/%d %H:%M")
            );
        }

        DiscordEmbedMessage {
            title: format!("Epic Games Store - {}", game.title),
            r#type: String::from("rich"),
            description,
            url,
            image: DiscordEmbedImage {
                url: game.key_images.first().unwrap().url.clone(),
            },
            footer: DiscordEmbedFooter { text: footer },
        }
    }
}
