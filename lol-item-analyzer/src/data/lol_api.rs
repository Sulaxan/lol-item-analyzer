use async_trait::async_trait;
use serde::Deserialize;

use crate::util;

use super::{Reader, LOL_ITEMS_KEY};

const LEAGUE_CDN_URL: &str = "http://ddragon.leagueoflegends.com/cdn/";
const LEAGUE_ITEM_ENDPOINT: &str = "/data/en_US/item.json";

pub struct LolApiReader;

#[async_trait()]
impl Reader for LolApiReader {
    async fn read<'a, T: Deserialize<'a>>(key: &str) -> Result<T, String> {
        match key {
            LOL_ITEMS_KEY => {
                let version = util::league_version::get_all_versions()
                    .await
                    .map_err(|e| format!("Could not get League versions: {}", e))?;
                let url = format!("{}{}{}", LEAGUE_CDN_URL, version[0], LEAGUE_ITEM_ENDPOINT);

                let response = reqwest::get(url)
                    .await
                    .map_err(|e| format!("Error fetching from URL: {}", e))?
                    .text()
                    .await
                    .map_err(|e| format!("Error obtaining request body: {}", e))?;

                //TODO: fix borrow lifetime error here
                serde_json::from_str(response.as_str()).map_err(|e| format!("Error parsing request body: {}", e))
            }
            _ => Err("Unrecognized key".to_string()),
        }
    }
}
