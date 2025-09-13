//! Key-value store.
//!
//! This module provides access to the host's key-value store. You should be able to manage keys,
//! values, and stores from within your functions.
//!
//! *Note:* stores are based on namespace set within a function. This allows for multiple stores per
//! function, persisting instance state.
use crate::region::Region;

unsafe extern "C" {
    fn kv_get(namespace_ptr: usize, key_ptr: usize) -> usize;
    fn kv_put(namespace_ptr: usize, key_ptr: usize, value_ptr: usize);
    fn kv_delete(namespace_ptr: usize, key_ptr: usize);
    fn kv_delete_store(namespace_ptr: usize);
    fn kv_commit(namespace_ptr: usize);
}

/// Key-value storage endpoints.
#[derive(Default)]
pub struct Storage;

impl Storage {
    /// Get a value from the key-value store.
    pub fn get(namespace: &str, key: &str) -> Option<Vec<u8>> {
        let namespace = Region::build(namespace.as_bytes());
        let namespace_ptr = &*namespace as *const Region;

        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;

        let value_ptr = unsafe { kv_get(namespace_ptr as usize, key_ptr as usize) };
        if value_ptr == 0 {
            return None;
        }

        unsafe { Some(Region::consume(value_ptr as *mut Region)) }
    }

    /// Put a value into the key-value store.
    pub fn put(namespace: &str, key: &str, value: &[u8]) {
        let namespace = Region::build(namespace.as_bytes());
        let namespace_ptr = &*namespace as *const Region;

        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;

        let value = Region::build(value);
        let value_ptr = &*value as *const Region;

        unsafe { kv_put(namespace_ptr as usize, key_ptr as usize, value_ptr as usize) };
    }

    /// Delete a value from the key-value store.
    pub fn delete(namespace: &str, key: &str) {
        let namespace = Region::build(namespace.as_bytes());
        let namespace_ptr = &*namespace as *const Region;

        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;

        unsafe { kv_delete(namespace_ptr as usize, key_ptr as usize) };
    }

    /// Delete the entire key-value store.
    pub fn delete_store(namespace: &str) {
        let namespace = Region::build(namespace.as_bytes());
        let namespace_ptr = &*namespace as *const Region;
        
        unsafe { kv_delete_store(namespace_ptr as usize) };
    }

    /// Persist the key-value store to the host storage.
    pub fn commit(namespace: &str) {
        let namespace = Region::build(namespace.as_bytes());
        let namespace_ptr = &*namespace as *const Region;

        unsafe { kv_commit(namespace_ptr as usize) };
    }
}
