use std::{collections::HashMap, fs, io};

use serde::{Deserialize, Serialize};

pub enum Error {
    FileNotExist,
    IoError { error: io::Error },
    ParseError { error: serde_json::Error },
    SerializeError { error: serde_json::Error },
}

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

impl Config {
    pub fn load(path: &str) -> Result<Self, Error> {
        let bytes = fs::read(path).map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                Error::FileNotExist
            } else {
                Error::IoError { error: e }
            }
        })?;

        let config: Config =
            serde_json::from_slice(bytes.as_slice()).map_err(|e| Error::ParseError { error: e })?;
        Ok(config)
    }

    pub fn save(config: &Config, path: &str) -> Result<(), Error> {
        let json = serde_json::to_vec(config).map_err(|e| Error::SerializeError { error: e })?;
        fs::write(path, json).map_err(|e| Error::IoError { error: e })
    }
}
