use serde::Deserialize;

use super::{Reader, LOL_ITEMS_KEY};

pub struct LolApiReader;

impl Reader for LolApiReader {
    fn read<'a, T: Deserialize<'a>>(key: &str) -> Result<T, String> {
        match key {
            LOL_ITEMS_KEY => match serde_json::from_str("") {
                Ok(val) => Ok(val),
                Err(err) => Err(err.to_string()),
            },
            _ => Err("Unrecognized key".to_string()),
        }
    }
}
