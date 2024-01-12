// Libs
use serde::{Deserialize, Serialize};

use super::epic_catalog::StoreGame;
use crate::webhooks::webhook::Webhook;
use crate::webhooks::*;

// Enums
#[derive(Clone, Deserialize, Serialize)]
pub enum WebhookType {
    Discord,
}

// Structs
#[derive(Default, Deserialize, Serialize)]
pub struct DataFile {
    pub webhooks: Vec<DataFileWebhook>,
    pub games: Vec<StoreGame>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DataFileWebhook {
    pub r#type: WebhookType,
    pub url: String,
    title: String,
}

impl DataFileWebhook {
    pub fn to_webhook(&self) -> impl Webhook {
        match self.r#type {
            WebhookType::Discord => discord::DiscordWebhook::new(&self.title, &self.url),
        }
    }
}
