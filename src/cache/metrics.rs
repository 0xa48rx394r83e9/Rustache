use std::sync::atomic::{AtomicUsize, Ordering};

pub struct CacheMetrics {
    hits: AtomicUsize,
    misses: AtomicUsize,
    writes: AtomicUsize,
    evictions: AtomicUsize,
    removals: AtomicUsize,
}

impl CacheMetrics {
    pub fn new() -> Self {
        CacheMetrics {
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
            writes: AtomicUsize::new(0),
            evictions: AtomicUsize::new(0),
            removals: AtomicUsize::new(0),
        }
    }

    pub fn increment_hits(&self) {
        self.hits.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_misses(&self) {
        self.misses.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_writes(&self) {
        self.writes.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_evictions(&self) {
        self.evictions.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_removals(&self) {
        self.removals.fetch_add(1, Ordering::SeqCst);
    }

    pub fn reset(&self) {
        self.hits.store(0, Ordering::SeqCst);
        self.misses.store(0, Ordering::SeqCst);
        self.writes.store(0, Ordering::SeqCst);
        self.evictions.store(0, Ordering::SeqCst);
        self.removals.store(0, Ordering::SeqCst);
    }

    pub fn get_hits(&self) -> usize {
        self.hits.load(Ordering::SeqCst)
    }

    pub fn get_misses(&self) -> usize {
        self.misses.load(Ordering::SeqCst)
    }

    pub fn get_writes(&self) -> usize {
        self.writes.load(Ordering::SeqCst)
    }

    pub fn get_evictions(&self) -> usize {
        self.evictions.load(Ordering::SeqCst)
    }

    pub fn get_removals(&self) -> usize {
        self.removals.load(Ordering::SeqCst)
    }
}