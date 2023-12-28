// Libs
use chrono::DateTime;

use super::catalog::StoreGame;

// Struct
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebhookMessage {
    pub embeds: Vec<EmbedMessage>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmbedMessage {
    pub title: String,
    pub r#type: String,
    pub description: String,
    pub url: String,
    pub image: EmbedImage,
    pub footer: Footer,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmbedImage {
    pub url: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Footer {
    pub text: String, // footer text
}

// Implementations
impl WebhookMessage {
    pub fn new(game: &StoreGame) -> Self {
        // Parse the date.
        let dt_init = DateTime::parse_from_rfc3339(&game.effective_date).unwrap();
        let dt_init = dt_init.with_timezone(&chrono::Local);
        let mut footer_text = format!("{}", dt_init.format("%Y/%m/%d %H:%M"));

        if let Some(dt) = game.expiry_date.clone() {
            let dt = DateTime::parse_from_rfc3339(&dt).unwrap();
            let dt = dt.with_timezone(&chrono::Local);
            footer_text = format!("{} - {}", footer_text, dt.format("%Y/%m/%d %H:%M"));
        }

        WebhookMessage {
            embeds: vec![EmbedMessage {
                title: format!("Epic Games Store - {}", game.title),
                r#type: String::from("rich"),
                description: format!("The game {} is free on the Store. ", game.title),
                url: format!("https://store.epicgames.com/en-US/p/{}", game.product_slug),
                image: EmbedImage {
                    url: game.key_images.first().unwrap().url.clone(),
                },
                footer: Footer { text: footer_text },
            }],
        }
    }
}
