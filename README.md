# SipHash.rs

SipHash.rs is a fast and efficient implementation of the SipHash hashing
algorithm in Rust, but without any dependencies on Rust's runtime.

By design, it only supports hashing sequential bytes from memory.

As it does *not* depend on the Rust runtime, it can be used in freestanding Rust
programs, including Kernels.

## Performance

On a 2.8ghz Intel Xeon Virtual Server, it takes 14ns (39 cycles) to warm up, and
then about 0.6ns (1.7 cycles) per byte.

Raw output:

    test bench_for_size_00000 ... bench:        14 ns/iter (+/- 0)
    test bench_for_size_00001 ... bench:        16 ns/iter (+/- 0)
    test bench_for_size_00002 ... bench:        16 ns/iter (+/- 0)
    test bench_for_size_00004 ... bench:        17 ns/iter (+/- 0)
    test bench_for_size_00008 ... bench:        20 ns/iter (+/- 0)
    test bench_for_size_00016 ... bench:        24 ns/iter (+/- 2)
    test bench_for_size_00032 ... bench:        35 ns/iter (+/- 0)
    test bench_for_size_00064 ... bench:        56 ns/iter (+/- 1)
    test bench_for_size_00128 ... bench:        98 ns/iter (+/- 1)
    test bench_for_size_00256 ... bench:       184 ns/iter (+/- 1)
    test bench_for_size_00512 ... bench:       343 ns/iter (+/- 14)
    test bench_for_size_01024 ... bench:       648 ns/iter (+/- 6)
    test bench_for_size_02048 ... bench:      1277 ns/iter (+/- 16)
    test bench_for_size_04096 ... bench:      2531 ns/iter (+/- 19)
    test bench_for_size_65536 ... bench:     39718 ns/iter (+/- 1702)

You can run these benchmarks yourself by using `make bench`.

## Usage

Add `siphash` as a dependency to your `Cargo.toml`

    [package]
    name = "foo"
    version = "0.0.0"
    authors = ["foo"]

    [[bin]]
    name = "foo"

    [dependencies.siphash]
    git = "https://github.com/utkarshkukreti/siphash.rs.git"

Now you can add `siphash` as `extern crate` in `src/foo.rs`

    extern crate siphash;

    fn main() {
        let sip = siphash::SipHasher::new();
        println!("hash for foo = {}", sip.hash(b"foo"));
    }

And now build and run it:

    $ cargo build
       Updating git repository `https://github.com/utkarshkukreti/siphash.rs.git`
      Compiling siphash v0.0.0 (https://github.com/utkarshkukreti/siphash.rs.git)
      Compiling foo v0.0.0 (file:/Users/utkarsh/dev/git/siphash.rs/examples)

    $ target/foo
    hash for foo = 15988776847138518036

## License

The reference implementation provided by the authors of SipHash is
licensed under the CC0 license, a public domain-like license. This library is
a direct port of it, and is also licensed under CC0.

More information: https://creativecommons.org/publicdomain/zero/1.0/
