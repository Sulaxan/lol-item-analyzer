use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::*;

pub struct LocalRw;

#[async_trait]
impl Reader for LocalRw {
    async fn read<'a, T: Deserialize<'a>>(key: &str) -> Result<T, String> {
        match key {
            LOL_ITEMS_KEY => match serde_json::from_str("") {
                Ok(val) => Ok(val),
                Err(err) => Err(err.to_string()),
            },
            _ => Err("Unrecognized key".to_string()),
        }
    }
}

#[async_trait]
impl Writer for LocalRw {
    async fn write<T: Serialize + Send>(key: &str, value: T) {
        todo!();
    }
}
