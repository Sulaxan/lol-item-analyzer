use serde::{Deserialize, Serialize};

pub mod local;
pub mod lol_api;

const LOL_ITEMS_KEY: &str = "lol_items";

pub trait Reader {
    fn read<'a, T: Deserialize<'a>>(key: &str) -> Result<T, String>;
}

pub trait Writer {
    fn write<T: Serialize>(key: &str, value: T);
}
