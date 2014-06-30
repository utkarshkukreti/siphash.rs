# SipHash.rs

This is a simple and fast implementation of the SipHash hashing algorithm.

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
