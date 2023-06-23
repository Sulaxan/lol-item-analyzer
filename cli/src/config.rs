use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub config_version: String,
    pub meta: Metadata,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
    pub item_version_files: HashMap<String, String>, // version => file path
    pub valid_versions: Vec<String>,
    pub valid_versions_last_update: u64,
}
