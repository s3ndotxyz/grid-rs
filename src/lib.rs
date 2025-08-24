//! # Slipstream Runtime SDK
//!
//! ## Overview
//!
//! This is a library for using host functions from the [*Slipstream
//! runtime*](https://github.com/s3ndotxyz/runtime). It's more or less a wrapper for linked WASM imports.
//!
//! ## Features
//!
//! * *kvs*. Provides access to the native in-runtime key-value store. You should be able tomanage
//! keys, values, and stores from within your functions. *Note:* stores are currently only
//! available, one to each function.
//! * *clock*. We've enabled secure time for you to do things like event scheduling.
//!
//! We have plans to enable a few other things such as message queues and web-sockets in the near
//! future. Stay tuned!
//!

pub mod clock;
pub mod kvs;
