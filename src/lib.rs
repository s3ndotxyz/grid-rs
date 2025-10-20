#![doc = include_str!("../README.md")]
#![no_main]
pub mod kvs;
pub mod ws;
#[doc(hidden)]
pub mod region;

pub use grid_rs_macros::main;

use region::Region;

/// Output buffer writer.
#[derive(Default)]
pub struct Output;

impl Output {
    pub fn write_all(data: &[u8]) -> usize {
        let region_ptr = Region::release_buffer(data.to_vec());
        region_ptr as usize
    }
}
