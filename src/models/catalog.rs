// Libs
use serde::{Deserialize, Serialize};

// Structs
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CatalogResponse {
    pub data: Data,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Data {
    #[serde(rename = "Catalog")]
    pub catalog: CatalogData,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CatalogData {
    #[serde(rename = "searchStore")]
    pub search_store: SearchStore,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SearchStore {
    pub elements: Vec<StoreGame>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StoreGame {
    pub id: String,
    pub title: String,
    pub description: String,

    #[serde(rename = "productSlug")]
    pub product_slug: String,

    #[serde(rename = "effectiveDate")]
    pub effective_date: String,

    #[serde(rename = "expiryDate")]
    pub expiry_date: Option<String>,

    #[serde(rename = "keyImages")]
    pub key_images: Vec<KeyImage>,

    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct KeyImage {
    pub r#type: String,
    pub url: String,
}
