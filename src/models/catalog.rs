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
    pub title: String,
    pub id: String,
    pub description: String,

    #[serde(rename = "productSlug")]
    pub product_slug: String,

    #[serde(rename = "effectiveDate")]
    pub effective_date: String,

    #[serde(rename = "expiryDate")]
    pub expiry_date: Option<String>,

    #[serde(rename = "viewableDate")]
    pub viewable_date: String,
    pub status: String,

    #[serde(rename = "keyImages")]
    pub key_images: Vec<KeyImage>,
    // url can be null
    pub url: Option<String>,
    pub price: Price,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct KeyImage {
    pub r#type: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Price {
    #[serde(rename = "totalPrice")]
    pub total_price: TotalPrice,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TotalPrice {
    #[serde(rename = "discountPrice")]
    pub discount_price: f32,

    #[serde(rename = "originalPrice")]
    pub original_price: f32,
}
