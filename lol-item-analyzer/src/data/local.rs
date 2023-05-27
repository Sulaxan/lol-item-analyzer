use serde::{Deserialize, Serialize};

use super::*;

pub struct LocalRw;

impl Reader for LocalRw {
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

impl Writer for LocalRw {
    fn write<T: Serialize>(key: &str, value: T) {}
}
