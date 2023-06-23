use std::fs;

use serde::Serialize;

pub fn write_json_file<T: Serialize>(path: &str, data: &T, pretty: bool) -> Result<(), String> {
    let val = serde_json::to_vec(data).map_err(|e| format!("Could not serialize data: {}", e))?;
    fs::write(path, val).map_err(|e| format!("Could not write to file: {}", e))
}
