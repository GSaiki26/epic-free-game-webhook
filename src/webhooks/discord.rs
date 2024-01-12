// Libs
use reqwest::Client;

use super::webhook::Webhook;
use crate::serialization::discord::{DiscordEmbedMessage, DiscordWebhookMessage};
use crate::serialization::epic_catalog::StoreGame;

// Structs
pub struct DiscordWebhook {
    url: String,
    title: String,
}

// Implementations
impl DiscordWebhook {
    pub fn new(title: &str, url: &str) -> Self {
        Self {
            url: String::from(url),
            title: String::from(title),
        }
    }
}

impl Webhook for DiscordWebhook {
    fn get_title(&self) -> &str {
        &self.title
    }

    async fn send_games(&self, games: &[StoreGame]) -> Box<Result<(), reqwest::Error>> {
        // Configure the request's body.
        let embeds: Vec<DiscordEmbedMessage> =
            games.iter().map(DiscordEmbedMessage::from).collect();

        for embed in embeds {
            println!("Webhook#{} Sending game #{}...", self.title, embed.title);
            let body = DiscordWebhookMessage {
                embeds: vec![embed],
            };
            let res = Client::new().post(&self.url).json(&body).send().await;
            if let Err(err) = res {
                return Box::new(Err(err));
            }
        }

        // Send the webhook message. req_body is a json.

        println!("Webhook#{} Done.", self.title);
        Box::new(Ok(()))
    }
}
