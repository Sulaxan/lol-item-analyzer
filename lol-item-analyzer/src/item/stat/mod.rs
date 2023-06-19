use serde::{Deserialize, Serialize};

pub mod transformer;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Stat {
    pub id: String,
    pub is_percent: bool,
}
