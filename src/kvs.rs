//! Key-value store.
//!
//! This module provides access to the host's key-value store. You should be able to manage keys,
//! values, and stores from within your functions.
//!
//! *Note:* stores are currently only available, one for each function.
use crate::region::Region;

unsafe extern "C" {
    fn kv_get(key_ptr: usize) -> usize;
    fn kv_put(key_ptr: usize, value_ptr: usize);
    fn kv_delete(key_ptr: usize);
    fn kv_delete_store();
    fn kv_commit();
}

/// Key-value storage endpoints.
#[derive(Default)]
pub struct Storage;

impl Storage {
    /// Get a value from the key-value store.
    pub fn get(key: &str) -> Option<Vec<u8>> {
        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;

        let value_ptr = unsafe { kv_get(key_ptr as usize) };
        if value_ptr == 0 {
            return None;
        }

        unsafe { Some(Region::consume(value_ptr as *mut Region)) }
    }

    /// Put a value into the key-value store.
    pub fn put(key: &str, value: &[u8]) {
        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;
        let value = Region::build(value);
        let value_ptr = &*value as *const Region;

        unsafe { kv_put(key_ptr as usize, value_ptr as usize) };
    }

    /// Delete a value from the key-value store.
    pub fn delete(key: &str) {
        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;

        unsafe { kv_delete(key_ptr as usize) };
    }

    /// Delete the entire key-value store.
    pub fn delete_store() {
        unsafe { kv_delete_store() };
    }

    /// Persist the key-value store to the host storage.
    pub fn commit() {
        unsafe { kv_commit() };
    }
}
