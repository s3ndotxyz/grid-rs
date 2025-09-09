//! # Slipstream Rust SDK (Grid)
//!
//! ## Overview
//!
//! This is a library for using host functions from the [*Slipstream
//! runtime*](https://github.com/s3ndotxyz/runtime). It's more or less a wrapper for linked WASM imports.
//!
//! ## Features
//!
//! * *kvs*. Provides access to the native in-runtime key-value store. You should be able tomanage keys, values, and stores from within your functions. *Note:* stores are currently only available, one to each function.
//! * *time*. We've enabled secure time for you to do things like event scheduling.
//!
//! We have plans to enable a few other things such as message queues and web-sockets in the near
//! future. Stay tuned!
//!

#![no_main]
pub mod time;
pub mod kvs;
pub mod region;

pub use macros::*;

use region::Region;

unsafe extern "C" {
    fn input_ptr() -> usize;
}

pub struct Input(Vec<u8>);

impl Input {
    pub fn read_all() -> *mut Region {
        let ptr = unsafe { input_ptr() };
        if ptr == 0 {
            return std::ptr::null_mut();
        }
        ptr as *mut Region
    }
}

impl AsRef<[u8]> for Input {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Default)]
pub struct Output;

impl Output {
    pub fn write_all(data: &[u8]) -> usize {
        let region_ptr = Region::release_buffer(data.to_vec());
        region_ptr as usize
    }
}
