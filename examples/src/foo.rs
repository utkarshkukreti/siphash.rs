extern crate siphash;

fn main() {
    let sip = siphash::SipHasher::new();
    println!("hash for foo = {}", sip.hash(b"foo"));
}
