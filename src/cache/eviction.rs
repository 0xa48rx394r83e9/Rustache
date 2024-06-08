use std::collections::HashMap;
use std::hash::Hash;

pub trait EvictionStrategy<K, V> {
    fn on_access(&self, key: &K, value: Option<&V>);
    fn evict<'a>(&self, data: &'a mut HashMap<K, V>) -> Option<(&'a K, &'a V)>;
}

pub struct LruEvictionStrategy<K>
where
    K: Eq + Hash,
{
    access_list: AsyncMutex<Vec<K>>,
}

impl<K> LruEvictionStrategy<K>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        LruEvictionStrategy {
            access_list: AsyncMutex::new(Vec::new()),
        }
    }
}

impl<K, V> EvictionStrategy<K, V> for LruEvictionStrategy<K>
where
    K: Eq + Hash + Clone,
{
    fn on_access(&self, key: &K, _value: Option<&V>) {
        let mut access_list = self.access_list.try_lock().unwrap();
        if let Some(index) = access_list.iter().position(|k| k == key) {
            access_list.remove(index);
        }
        access_list.push(key.clone());
    }

    fn evict<'a>(&self, data: &'a mut HashMap<K, V>) -> Option<(&'a K, &'a V)> {
        let mut access_list = self.access_list.try_lock().unwrap();
        if let Some(key) = access_list.pop() {
            if let Some(value) = data.get(&key) {
                return Some((&key, value));
            }
        }
        None
    }
}