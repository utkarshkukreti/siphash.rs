#![feature(test)]
extern crate test;
extern crate siphash;

use siphash::SipHasher;

macro_rules! bench_for_size {
    ($f:ident, $size:expr) => {
        #[bench]
        fn $f(b: &mut test::Bencher) {
            use std::iter::repeat;

            let chunk = repeat(b'.').take($size).collect::<Vec<_>>();
            let sip = SipHasher::new();

            b.iter(|| sip.hash(chunk.as_slice()));
        }
    }
}

bench_for_size!(bench_for_size_00000, 0);
bench_for_size!(bench_for_size_00001, 1);
bench_for_size!(bench_for_size_00002, 2);
bench_for_size!(bench_for_size_00004, 4);
bench_for_size!(bench_for_size_00008, 8);
bench_for_size!(bench_for_size_00016, 16);
bench_for_size!(bench_for_size_00032, 32);
bench_for_size!(bench_for_size_00064, 64);
bench_for_size!(bench_for_size_00128, 128);
bench_for_size!(bench_for_size_00256, 256);
bench_for_size!(bench_for_size_00512, 512);
bench_for_size!(bench_for_size_01024, 1024);
bench_for_size!(bench_for_size_02048, 2048);
bench_for_size!(bench_for_size_04096, 4096);
bench_for_size!(bench_for_size_65536, 65536);
