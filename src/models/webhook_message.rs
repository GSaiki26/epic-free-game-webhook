// Libs
use super::catalog::KeyImage;

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
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmbedImage {
    pub url: String,
}

// Implementations
impl WebhookMessage {
    pub fn new(game_name: String, product_slug: String, image: &KeyImage) -> Self {
        WebhookMessage {
            embeds: vec![EmbedMessage {
                title: format!("Epic Games Store - {}", game_name),
                r#type: String::from("rich"),
                description: format!("The game {} is free on the Store. ", game_name),
                url: format!("https://store.epicgames.com/en-US/p/{}", product_slug),
                image: EmbedImage {
                    url: image.url.clone(),
                },
            }],
        }
    }
}
