use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// Thread-safe [HashMap] wrapper to map each native callback pointer to its respective Rust function.
pub(crate) struct CronetCallbacks<K, V> {
    map: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V> CronetCallbacks<K, V> {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get the underlying [HashMap] wrapped in an [Arc] and [Mutex].
    pub fn map(&self) -> &Arc<Mutex<HashMap<K, V>>> {
        &self.map
    }
}
