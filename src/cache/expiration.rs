use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

use crate::utils::async_utils::AsyncMutex;

pub trait ExpirationStrategy {
    fn on_access<K>(&self, key: &K);
    fn on_write<K>(&self, key: &K);
    fn check_expiration<K, V>(&self, key: &K, data: &mut HashMap<K, V>) -> bool
    where
        K: Eq + Hash + Clone;
}

pub struct TtlExpirationStrategy {
    ttl: Duration,
    expiration_times: AsyncMutex<HashMap<String, Instant>>,
}

impl TtlExpirationStrategy {
    pub fn new(ttl: Duration) -> Self {
        TtlExpirationStrategy {
            ttl,
            expiration_times: AsyncMutex::new(HashMap::new()),
        }
    }
}

impl ExpirationStrategy for TtlExpirationStrategy {
    fn on_access<K>(&self, key: &K) {
        let key_string = format!("{:?}", key);
        let mut expiration_times = self.expiration_times.try_lock().unwrap();
        if let Some(expiration_time) = expiration_times.get_mut(&key_string) {
            *expiration_time = Instant::now() + self.ttl;
        }
    }

    fn on_write<K>(&self, key: &K) {
        let key_string = format!("{:?}", key);
        let mut expiration_times = self.expiration_times.try_lock().unwrap();
        expiration_times.insert(key_string, Instant::now() + self.ttl);
    }

    fn check_expiration<K, V>(&self, key: &K, data: &mut HashMap<K, V>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        let key_string = format!("{:?}", key);
        let mut expiration_times = self.expiration_times.try_lock().unwrap();
        if let Some(expiration_time) = expiration_times.get(&key_string) {
            if expiration_time <= &Instant::now() {
                data.remove(key);
                expiration_times.remove(&key_string);
                return true;
            }
        }
        false
    }
}