#![crate_id = "siphash"]
#![no_std]

pub struct SipHasher {
    k0: u64,
    k1: u64
}

impl SipHasher {
    pub fn new() -> SipHasher {
        SipHasher::new_with_keys(0, 0)
    }

    pub fn new_with_keys(k0: u64, k1: u64) -> SipHasher {
        SipHasher {
            k0: k0,
            k1: k1
        }
    }
}
