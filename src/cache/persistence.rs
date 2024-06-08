use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

use crate::utils::error_handling::CacheError;
use crate::utils::serialization::{deserialize_from_file, serialize_to_file};

pub struct PersistentCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    cache: Cache<K, V>,
    file_path: String,
}

impl<K, V> PersistentCache<K, V>
where
    K: Eq + Hash + Clone + serde::Serialize + for<'a> serde::Deserialize<'a>,
    V: Clone + serde::Serialize + for<'a> serde::Deserialize<'a>,
{
    pub fn new(cache: Cache<K, V>, file_path: &str) -> Self {
        PersistentCache {
            cache,
            file_path: file_path.to_string(),
        }
    }

    pub async fn load(&self) -> Result<(), CacheError> {
        if Path::new(&self.file_path).exists() {
            let data: HashMap<K, V> = deserialize_from_file(&self.file_path)?;
            for (key, value) in data {
                self.cache.set(key, value).await?;
            }
        }
        Ok(())
    }

    pub async fn persist(&self) -> Result<(), CacheError> {
        let data = self.cache.get_all().await;
        serialize_to_file(&data, &self.file_path)?;
        Ok(())
    }
}