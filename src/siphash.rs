#![crate_id = "siphash"]
#![no_std]

pub struct SipHasher {
    k0: u64,
    k1: u64
}
