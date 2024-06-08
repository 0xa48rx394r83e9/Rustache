# ✧༺✦ Rustache ✦༺✧

Rustache is a high-performance, in-memory caching system implemented in Rust. It provides a robust and efficient solution for caching key-value pairs in memory, with support for various advanced features such as eviction strategies, expiration, persistence, sharding, and layering.

## ✧༺✦ Features ✦༻✧

- **Thread-safe**: Rustache uses thread-safe data structures and synchronization primitives to ensure safe concurrent access to the cache.
- **Eviction Strategies**: Rustache supports different eviction strategies, such as Least Recently Used (LRU), to efficiently manage the cache size and remove items when the cache reaches its capacity.
- **Expiration**: Cached items can be assigned an expiration time, after which they are automatically removed from the cache. Rustache supports time-to-live (TTL) based expiration.
- **Persistence**: Rustache allows persisting the cached data to disk and loading it back into memory when the cache is initialized. This enables cache recovery across application restarts.
- **Sharding**: Rustache supports sharding the cache data across multiple cache instances based on the hash of the keys. This enables distributing the cache load and improves scalability.
- **Layering**: Rustache implements a two-level caching strategy with an L1 cache and an L2 cache. The L1 cache is smaller but faster, while the L2 cache is larger but slower. This allows optimizing cache performance based on access patterns.
- **Metrics**: Rustache tracks various cache metrics, such as hits, misses, writes, evictions, and removals, providing insights into cache performance and usage.
- **Serialization**: Cached data can be serialized and deserialized using the Serde library, enabling easy storage and retrieval of structured data.
- **Error Handling**: Rustache uses custom error types and provides meaningful error messages for improved debugging and error handling.

## ✧༺✦ Getting Started ✦༻✧

To use Rustache in your Rust project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
rustache = "0.1.0"
```

Then, you can create a new cache instance and start using it in your code:

```rust
use rustache::cache::Cache;

fn main() {
    let cache = Cache::new(100); // Create a new cache with a capacity of 100 items

    // Use the cache
}
```

## ✧༺✦ Examples ✦༻✧

Here are a few examples of how you can use Rustache in your Rust code:

### Basic Usage

```rust
use rustache::cache::Cache;

async fn main() {
    let cache = Cache::new(100);

    // Set a value in the cache
    cache.set("key1", "value1").await;

    // Get a value from the cache
    let value = cache.get("key1").await;
    println!("Value for key1: {:?}", value);

    // Remove a value from the cache
    cache.remove("key1").await;
}
```

### Cache with Eviction Strategy

```rust
use rustache::cache::{Cache, LruEvictionStrategy};

async fn main() {
    let eviction_strategy = LruEvictionStrategy::new();
    let cache = Cache::new(eviction_strategy, 100);

    // Use the cache
}
```

### Cache with Persistence

```rust
use rustache::cache::PersistentCache;

async fn main() {
    let cache = PersistentCache::new("cache.json");

    // Load the cache from disk
    cache.load().await;

    // Use the cache

    // Persist the cache to disk
    cache.persist().await;
}
```

## ✧༺✦ License ✦༻✧

Rustache is licensed under the [MIT License](LICENSE).

---

✦༺✧ Happy Caching with Rustache! ✧༻✦
