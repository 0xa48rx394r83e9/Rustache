use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use crate::cache::eviction::EvictionStrategy;
use crate::cache::expiration::ExpirationStrategy;
use crate::cache::metrics::CacheMetrics;
use crate::utils::async_utils::{lock_and_execute, AsyncMutex};
use crate::utils::error_handling::CacheError;

pub struct Cache<K, V, E, X>
where
    K: Eq + Hash,
    E: EvictionStrategy<K, V>,
    X: ExpirationStrategy,
{
    data: AsyncMutex<HashMap<K, V>>,
    eviction_strategy: E,
    expiration_strategy: X,
    metrics: Arc<CacheMetrics>,
    capacity: usize,
}

impl<K, V, E, X> Cache<K, V, E, X>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    E: EvictionStrategy<K, V> + Send + Sync + 'static,
    X: ExpirationStrategy + Send + Sync + 'static,
{
    pub fn new(eviction_strategy: E, expiration_strategy: X, capacity: usize) -> Self {
        Cache {
            data: AsyncMutex::new(HashMap::new()),
            eviction_strategy,
            expiration_strategy,
            metrics: Arc::new(CacheMetrics::new()),
            capacity,
        }
    }

    pub async fn get(&self, key: &K) -> Result<Option<V>, CacheError> {
        let metrics = self.metrics.clone();
        lock_and_execute(&self.data, |data| {
            let value = data.get(key).cloned();
            if let Some(ref v) = value {
                metrics.increment_hits();
                self.eviction_strategy.on_access(key, Some(v));
                self.expiration_strategy.on_access(key);
            } else {
                metrics.increment_misses();
            }
            Ok(value)
        })
        .await
    }

    pub async fn set(&self, key: K, value: V) -> Result<(), CacheError> {
        lock_and_execute(&self.data, |data| {
            if data.len() >= self.capacity {
                if let Some((evicted_key, _)) = self.eviction_strategy.evict(data) {
                    data.remove(&evicted_key);
                    self.metrics.increment_evictions();
                }
            }
            data.insert(key.clone(), value);
            self.expiration_strategy.on_write(&key);
            self.metrics.increment_writes();
            Ok(())
        })
        .await
    }

    pub async fn remove(&self, key: &K) -> Result<Option<V>, CacheError> {
        lock_and_execute(&self.data, |data| {
            let value = data.remove(key);
            if value.is_some() {
                self.metrics.increment_removals();
            }
            Ok(value)
        })
        .await
    }

    pub async fn clear(&self) -> Result<(), CacheError> {
        lock_and_execute(&self.data, |data| {
            data.clear();
            self.metrics.reset();
            Ok(())
        })
        .await
    }

    pub async fn len(&self) -> usize {
        lock_and_execute(&self.data, |data| data.len()).await
    }

    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}