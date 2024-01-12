// Libs
use crate::serialization::epic_catalog::StoreGame;

// Traits
pub trait Webhook {
    /// A method to get the webhook's name.
    fn get_title(&self) -> &str;

    /// A method to send the game using the child webhook.
    fn send_games(
        &self,
        games: &[StoreGame],
    ) -> impl std::future::Future<Output = Box<Result<(), reqwest::Error>>> + std::marker::Send;
}
