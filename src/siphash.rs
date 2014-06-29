#![crate_id = "siphash"]
#![feature(globs, phase)]
#![no_std]

// For tests.
#[cfg(test)] #[phase(plugin, link)] extern crate std;
#[cfg(test)] extern crate native;

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

#[cfg(test)]
mod test {
    #[test]
    fn test_1_plus_1() {
        assert_eq!(1u + 1, 2);
    }
}
