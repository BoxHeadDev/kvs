use kvs::KvStore;

//
// Test 1: Should get previously stored value
//
#[test]
fn get_stored_value() {
    let mut store = KvStore::new(); // Create a new key-value store

    // Insert two key-value pairs
    store.set("key1".to_owned(), "value1".to_owned());
    store.set("key2".to_owned(), "value2".to_owned());

    // Ensure the stored values can be retrieved correctly
    assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    assert_eq!(store.get("key2".to_owned()), Some("value2".to_owned()));
}

//
// Test 2: Should overwrite existent value
//
#[test]
fn overwrite_value() {
    let mut store = KvStore::new(); // Create a new key-value store

    // Set an initial value
    store.set("key1".to_owned(), "value1".to_owned());
    // Confirm it was stored
    assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));

    // Overwrite the existing value for "key1"
    store.set("key1".to_owned(), "value2".to_owned());
    // Confirm that the value has been updated
    assert_eq!(store.get("key1".to_owned()), Some("value2".to_owned()));
}

//
// Test 3: Should get `None` when getting a non-existent key
//
#[test]
fn get_non_existent_value() {
    let mut store = KvStore::new(); // Create a new key-value store

    // Set a single key
    store.set("key1".to_owned(), "value1".to_owned());

    // Try to get a value for a key that hasn't been set
    assert_eq!(store.get("key2".to_owned()), None); // Should return None
}

//
// Test 4: Should remove a key
//
#[test]
fn remove_key() {
    let mut store = KvStore::new(); // Create a new key-value store

    // Set a key-value pair
    store.set("key1".to_owned(), "value1".to_owned());

    // Remove the key from the store
    store.remove("key1".to_owned());

    // After removal, getting the key should return None
    assert_eq!(store.get("key1".to_owned()), None);
}
