unsafe extern "C" {
    fn time() -> u64;
}

pub fn now() -> u64 {
    unsafe { time() }
}

