use crate::region::Region;

unsafe extern "C" {
    fn kv_get(namespace_ptr: usize, key_ptr: usize) -> usize;
    fn kv_set(namespace_ptr: usize, key_ptr: usize, value_ptr: usize);
    fn kv_delete(namespace_ptr: usize, key_ptr: usize);
    fn kv_delete_store(namespace_ptr: usize);
    fn kv_commit(namespace_ptr: usize);
}

#[derive(Default)]
pub struct Storage;

impl Storage {
    pub fn get(&self, namespace: &str, key: &str) -> Option<Vec<u8>> {
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

    pub fn set(&self, namespace: &str, key: &str, value: &[u8]) {
        let namespace = Region::build(namespace.as_bytes());
        let namespace_ptr = &*namespace as *const Region;
        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;
        let value = Region::build(value);
        let value_ptr = &*value as *const Region;

        unsafe { kv_set(namespace_ptr as usize, key_ptr as usize, value_ptr as usize) };
    }

    pub fn delete(&self, namespace: &str, key: &str) {
        let namespace = Region::build(namespace.as_bytes());
        let namespace_ptr = &*namespace as *const Region;
        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;

        unsafe { kv_delete(namespace_ptr as usize, key_ptr as usize) };
    }

    pub fn delete_store(&self, namespace: &str) {
        let namespace = Region::build(namespace.as_bytes());
        let namespace_ptr = &*namespace as *const Region;

        unsafe { kv_delete_store(namespace_ptr as usize) };
    }

    pub fn commit(&self, namespace: &str) {
        let namespace = Region::build(namespace.as_bytes());
        let namespace_ptr = &*namespace as *const Region;

        unsafe { kv_commit(namespace_ptr as usize) };
    }
}
