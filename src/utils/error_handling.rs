use std::fmt;

#[derive(Debug)]
pub enum CacheError {
    SerializationError(String),
    DeserializationError(String),
    PersistenceError(String),
    InvalidKey,
    CacheFullError,
    // Add more error variants as needed
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            CacheError::DeserializationError(msg) => write!(f, "Deserialization Error: {}", msg),
            CacheError::PersistenceError(msg) => write!(f, "Persistence Error: {}", msg),
            CacheError::InvalidKey => write!(f, "Invalid Key"),
            CacheError::CacheFullError => write!(f, "Cache is full"),
            // Add more error variants and their corresponding display formatting
        }
    }
}

impl std::error::Error for CacheError {}