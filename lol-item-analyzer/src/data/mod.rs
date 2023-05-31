use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod local;
pub mod lol_api;

const LOL_ITEMS_KEY: &str = "lol_items";

#[async_trait]
pub trait Reader {
    async fn read<'a, T: Deserialize<'a>>(key: &str) -> Result<T, String>;
}

#[async_trait]
pub trait Writer {
    async fn write<T: Serialize + Send>(key: &str, value: T);
}
