#![crate_name = "siphash"]
#![crate_type = "lib"]
#![feature(globs, macro_rules)]
#![no_std]

extern crate core;

use core::prelude::*;
use core::intrinsics::transmute;
use core::raw::Slice;

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
        let mut b = len as u64 << 56;

        v3 ^= self.k1;
        v2 ^= self.k0;
        v1 ^= self.k1;
        v0 ^= self.k0;

        let new_slice: &[u64] = unsafe {
            transmute(Slice {
                data: bytes.as_ptr(),
                len: len / 8
            })
        };

        for m in new_slice.iter().map(|n| n.to_le()) {
            v3 ^= m;
            round!(v0, v1, v2, v3);
            round!(v0, v1, v2, v3);
            v0 ^= m;
        }

        macro_rules! k {
            ($($i:expr)+) => {{
                unsafe {
                    $(b |= *bytes.unsafe_ref(len - left + $i) as u64 << 8 * $i;)+
                }
            }}
        }

        match left {
            7 => k!(0 1 2 3 4 5 6),
            6 => k!(0 1 2 3 4 5),
            5 => k!(0 1 2 3 4),
            4 => k!(0 1 2 3),
            3 => k!(0 1 2),
            2 => k!(0 1),
            1 => k!(0),
            _ => {}
        };

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
