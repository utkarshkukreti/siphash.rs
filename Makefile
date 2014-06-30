RUST_FLAGS = -O --out-dir target -L target

lib: src/siphash.rs
	mkdir -p target
	rustc $(RUST_FLAGS) src/siphash.rs

test: lib
	rustc $(RUST_FLAGS) --test src/siphash_test.rs
	target/siphash_test

bench: lib
	rustc $(RUST_FLAGS) --test src/siphash_test.rs
	target/siphash_test --bench
