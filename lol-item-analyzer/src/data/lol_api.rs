use std::collections::HashMap;

use serde_json::Value;

use crate::item::Item;

const LEAGUE_API_URL: &str = "https://ddragon.leagueoflegends.com/api";
const LEAGUE_CDN_URL: &str = "http://ddragon.leagueoflegends.com/cdn";
const LEAGUE_API_VERSIONS_ENDPOINT: &str = "/versions.json";
const LEAGUE_CDN_ITEM_ENDPOINT: &str = "/data/en_US/item.json";
const ITEMS_DATA_KEY: &str = "data";

pub struct LolApi;

impl LolApi {
    /// Obtains all valid League of Legends versions from the League API.
    pub async fn get_all_versions() -> Result<Vec<String>, String> {
        let response = reqwest::get(format!(
            "{}{}",
            LEAGUE_API_URL, LEAGUE_API_VERSIONS_ENDPOINT
        ))
        .await
        .map_err(|e| format!("Error with response: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Could not get text: {}", e))?;

        serde_json::from_str(response.as_str())
            .map_err(|e| format!("Error while parsing JSON string: {}", e))
    }

    /// Obtains the latest League of Legends version.
    pub async fn get_latest_version() -> Result<String, String> {
        LolApi::get_all_versions()
            .await?
            .get(0)
            .cloned()
            .ok_or("No versions available".to_owned())
    }

    /// Obtains all items from the League of Legends CDN. This produces the raw content containing
    /// extra metadata.
    pub async fn get_raw_items(version: &str) -> Result<HashMap<String, Value>, String> {
        let url = format!(
            "{}/{}{}",
            LEAGUE_CDN_URL, version, LEAGUE_CDN_ITEM_ENDPOINT
        );

        let response = reqwest::get(url)
            .await
            .map_err(|e| format!("Error fetching from URL: {}", e))?
            .text()
            .await
            .map_err(|e| format!("Error obtaining request body: {}", e))?;

        let raw_items: HashMap<String, Value> = serde_json::from_str(response.as_str())
            .map_err(|e| format!("Error parsing request body: {}", e))?;

        Ok(raw_items)
    }

    /// Obtains all items from the League of Legends CDN. Returned result is a map of item IDs
    /// mapped to Item structs.
    pub async fn get_items(version: &str) -> Result<HashMap<String, Item>, String> {
        let raw_items = LolApi::get_raw_items(version).await?;

        let data = raw_items
            .get(ITEMS_DATA_KEY)
            .ok_or("No data in items response")?
            .to_owned();
        let items: HashMap<String, Item> =
            serde_json::from_value(data).map_err(|e| format!("Could not parse items: {}", e))?;

        Ok(items)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn get_all_versions_works() {
        let result = LolApi::get_all_versions().await.unwrap();

        assert_ne!(result.len(), 0, "Result is empty list");

        println!("{:#?}", result);
    }

    #[tokio::test]
    async fn get_items_works() {
        let latest_version = LolApi::get_latest_version().await.unwrap();
        let result = LolApi::get_items(latest_version.as_str()).await.unwrap();

        assert_ne!(result.len(), 0, "Result is empty list");

        println!("{:#?}", result);
    }
}
