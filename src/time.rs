//! Secure time (partially).
//!
//! Right now, we've only exposed a single function to get the current time from the host's trusted
//! source. We'll be adding more functionality in the near future to enable things like event
//! scheduling.
unsafe extern "C" {
    fn time() -> u64;
}

/// Get the current time from the host's trusted source.
/// Minimally trust this.
pub fn now() -> u64 {
    unsafe { time() }
}

