use crate::cache::Cache;

pub struct LayeredCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    l1_cache: Cache<K, V>,
    l2_cache: Cache<K, V>,
}

impl<K, V> LayeredCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(l1_capacity: usize, l2_capacity: usize) -> Self {
        LayeredCache {
            l1_cache: Cache::new(l1_capacity),
            l2_cache: Cache::new(l2_capacity),
        }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        match self.l1_cache.get(key).await {
            Some(value) => Some(value),
            None => {
                let value = self.l2_cache.get(key).await;
                if let Some(v) = &value {
                    self.l1_cache.set(key.clone(), v.clone()).await;
                }
                value
            }
        }
    }

    pub async fn set(&self, key: K, value: V) {
        self.l1_cache.set(key.clone(), value.clone()).await;
        self.l2_cache.set(key, value).await;
    }

    pub async fn remove(&self, key: &K) -> Option<V> {
        self.l1_cache.remove(key).await;
        self.l2_cache.remove(key).await
    }
}