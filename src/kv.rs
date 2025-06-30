// Import the HashMap type from the standard library's collections module.
// HashMap is used to store key-value pairs efficiently.
use std::collections::HashMap;

// Define a struct named `KvStore` which represents a simple key-value store.
// The #[derive(Default)] macro automatically implements the Default trait,
// allowing us to create a default instance of KvStore using KvStore::default().
#[derive(Default)]
pub struct KvStore {
    // Internal storage of the KvStore using a HashMap with String keys and values.
}

impl KvStore {
    /// Creates and returns a new, empty KvStore.
    /// This function initializes the internal HashMap.
    pub fn new() -> KvStore {
        // Create a new empty HashMap
    }

    /// Inserts or updates a key-value pair in the store.
    ///
    /// If the key already exists in the map, its value is overwritten.
    pub fn set(&mut self, key: String, value: String) {
        // Insert or overwrite the key-value pair
    }

    /// Retrieves the value associated with a key from the store.
    ///
    /// Returns an `Option<String>`:
    /// - `Some(value)` if the key exists
    /// - `None` if the key is not present
    ///
    /// `.cloned()` is used to return a copy of the value rather than a reference.
    /// This avoids lifetime issues and lets the caller own the returned value.
    pub fn get(&self, key: String) -> Option<String> {
        // Look up the value and clone it if found
    }

    /// Removes a key-value pair from the store.
    /// If the key doesn't exist, nothing happens.
    pub fn remove(&mut self, key: String) {
        // Remove the key from the HashMap if it exists
    }
}
