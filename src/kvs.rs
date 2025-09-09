use crate::region::Region;

unsafe extern "C" {
    fn kv_get(key_ptr: usize) -> usize;
    fn kv_set(key_ptr: usize, value_ptr: usize);
    fn kv_delete(key_ptr: usize);
    fn kv_delete_store();
    fn kv_commit();
}

#[derive(Default)]
pub struct Storage;

impl Storage {
    pub fn get(key: &str) -> Option<Vec<u8>> {
        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;

        let value_ptr = unsafe { kv_get(key_ptr as usize) };
        if value_ptr == 0 {
            return None;
        }

        unsafe { Some(Region::consume(value_ptr as *mut Region)) }
    }

    pub fn set(key: &str, value: &[u8]) {
        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;
        let value = Region::build(value);
        let value_ptr = &*value as *const Region;

        unsafe { kv_set(key_ptr as usize, value_ptr as usize) };
    }

    pub fn delete(key: &str) {
        let key = Region::build(key.as_bytes());
        let key_ptr = &*key as *const Region;

        unsafe { kv_delete(key_ptr as usize) };
    }

    pub fn delete_store() {
        unsafe { kv_delete_store() };
    }

    pub fn commit() {
        unsafe { kv_commit() };
    }
}
