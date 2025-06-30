use serde::{Deserialize, Serialize};

/// Represents a command in the key-value store.
///
/// The `Command` enum defines the supported operations:
/// - `Set`: Stores a value for a given key.
/// - `Remove`: Deletes the key and its associated value from the store.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// Set a value for the given key.
    ///
    /// # Fields
    /// - `key`: The key to associate with the value.
    /// - `value`: The value to store.
    Set {
        /// The key to associate with the value.
        key: String,
        /// The value to store.
        value: String,
    },

    /// Remove the value associated with the given key.
    ///
    /// # Fields
    /// - `key`: The key to remove from the store.
    Remove {
        /// The key to remove from the store.
        key: String,
    },
}

impl Command {
    /// Creates a new `Set` command with the specified key and value.
    ///
    /// # Arguments
    ///
    /// * `key` - A `String` representing the key to be set.
    /// * `value` - A `String` representing the value to associate with the key.
    ///
    /// # Example
    ///
    /// `` `
    /// # use kvs::Command;
    /// let cmd = Command::set("name".to_string(), "Alice".to_string());
    /// ```
    pub fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    /// Creates a new `Remove` command for the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - A `String` representing the key to be removed.
    ///
    /// # Example
    ///
    /// ```
    /// # use kvs::Command;
    /// let cmd = Command::remove("name".to_string());
    /// ```
    pub fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}
