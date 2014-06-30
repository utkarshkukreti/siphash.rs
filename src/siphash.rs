#![crate_id = "siphash"]
#![crate_type = "lib"]
#![feature(globs, macro_rules)]
#![no_std]

extern crate core;

use core::prelude::*;

pub struct SipHasher {
    k0: u64,
    k1: u64
}

macro_rules! rotl {
    ($x:expr, $b:expr) => { $x << $b | $x >> (64 - $b) }
}

macro_rules! round {
    ($v0:ident, $v1:ident, $v2:ident, $v3:ident) => {{
        $v0 += $v1; $v1 = rotl!($v1, 13); $v1 ^= $v0; $v0 = rotl!($v0, 32);
        $v2 += $v3; $v3 = rotl!($v3, 16); $v3 ^= $v2;
        $v0 += $v3; $v3 = rotl!($v3, 21); $v3 ^= $v0;
        $v2 += $v1; $v1 = rotl!($v1, 17); $v1 ^= $v2; $v2 = rotl!($v2, 32);
    }}
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

    pub fn hash(&self, bytes: &[u8]) -> u64 {
        let len = bytes.len();

        let mut v0 = 0x736f6d6570736575;
        let mut v1 = 0x646f72616e646f6d;
        let mut v2 = 0x6c7967656e657261;
        let mut v3 = 0x7465646279746573;

        let left = len & 7;
        let end = len - left;
        let mut b: u64 = len as u64 << 56;

        v3 ^= self.k1;
        v2 ^= self.k0;
        v1 ^= self.k1;
        v0 ^= self.k0;

        for i in core::iter::range_step(0, end, 8) {
            let m = unsafe {
                (*(bytes.as_ptr().offset(i as int) as *const u64)).to_le()
            };

            v3 ^= m;
            round!(v0, v1, v2, v3);
            round!(v0, v1, v2, v3);
            v0 ^= m;
        }

        for i in range(end, len) {
            unsafe {
                b |= *bytes.unsafe_ref(i) as u64 << 8 * i
            }
        }

        v3 ^= b;

        round!(v0, v1, v2, v3);
        round!(v0, v1, v2, v3);

        v0 ^= b;
        v2 ^= 0xff;

        round!(v0, v1, v2, v3);
        round!(v0, v1, v2, v3);
        round!(v0, v1, v2, v3);
        round!(v0, v1, v2, v3);

        v0 ^ v1 ^ v2 ^ v3
    }
}
