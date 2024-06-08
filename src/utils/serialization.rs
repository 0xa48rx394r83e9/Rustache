use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::utils::error_handling::CacheError;

pub fn serialize_to_file<T: Serialize>(data: &T, file_path: &str) -> Result<(), CacheError> {
    let serialized = serde_json::to_string(data)
        .map_err(|e| CacheError::SerializationError(e.to_string()))?;

    fs::write(file_path, serialized)
        .map_err(|e| CacheError::PersistenceError(e.to_string()))?;

    Ok(())
}

pub fn deserialize_from_file<T: for<'a> Deserialize<'a>>(file_path: &str) -> Result<T, CacheError> {
    let file_content = fs::read_to_string(file_path)
        .map_err(|e| CacheError::PersistenceError(e.to_string()))?;

    let deserialized: T = serde_json::from_str(&file_content)
        .map_err(|e| CacheError::DeserializationError(e.to_string()))?;

    Ok(deserialized)
}

pub fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}