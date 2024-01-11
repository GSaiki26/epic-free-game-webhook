// Libs
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Structs
#[derive(Clone, Deserialize, Serialize)]
pub struct CatalogResponse {
    pub data: Data,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename = "Catalog")]
    pub catalog: CatalogData,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CatalogData {
    #[serde(rename = "searchStore")]
    pub search_store: SearchStore,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SearchStore {
    pub elements: Vec<StoreGame>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct StoreGame {
    pub id: String,
    pub title: String,
    pub description: String,

    #[serde(rename = "productSlug")]
    pub product_slug: Option<String>,

    pub promotions: Promotions,

    #[serde(rename = "keyImages")]
    pub key_images: Vec<KeyImage>,

    pub url: Option<String>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Promotions {
    #[serde(rename = "promotionalOffers")]
    pub promotional_offers: Vec<PromotionalOffer>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct PromotionalOffer {
    #[serde(rename = "promotionalOffers")]
    pub promotional_offers: Vec<Offer>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Offer {
    #[serde(rename = "startDate")]
    pub start_date: String,

    #[serde(rename = "endDate")]
    pub end_date: String,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct KeyImage {
    pub r#type: String,
    pub url: String,
}

// Functions
/**
 * A method to get the current catalog from EpicGames.
*/
pub async fn get_catalog() -> CatalogResponse {
    println!("Getting the current catalog...");

    // Get the current catalog.
    let url = std::env::var("CATALOG_URL").expect("CATALOG_URL not found.");
    Client::new()
        .get(url)
        .send()
        .await
        .expect("Couldn\'t connect to epicGames.")
        .json::<CatalogResponse>()
        .await
        .expect("Couldn\'t parse the CATALOG_URL body.")
}
