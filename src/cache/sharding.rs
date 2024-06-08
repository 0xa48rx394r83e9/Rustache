use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::cache::Cache;

pub struct ShardedCache<K, V>
where
    K: Eq + Hash,
{
    shards: Vec<Arc<Cache<K, V>>>,
}

impl<K, V> ShardedCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new(num_shards: usize, cache_factory: impl Fn() -> Cache<K, V>) -> Self {
        let shards = (0..num_shards)
            .map(|_| Arc::new(cache_factory()))
            .collect();
        ShardedCache { shards }
    }

    fn get_shard_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.shards.len()
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        let index = self.get_shard_index(key);
        self.shards[index].get(key).await
    }

    pub async fn set(&self, key: K, value: V) {
        let index = self.get_shard_index(&key);
        self.shards[index].set(key, value).await;
    }

    pub async fn remove(&self, key: &K) -> Option<V> {
        let index = self.get_shard_index(key);
        self.shards[index].remove(key).await
    }
}