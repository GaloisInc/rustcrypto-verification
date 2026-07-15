# SHA-1

Verification of SHA-1 from the [`sha1`](https://crates.io/crates/sha1/0.10.6)
crate against the [SHA-1
specification](../cryptol-specs/Primitive/Keyless/Hash/SHA1/Specification.cry)
from `cryptol-specs`.

Currently only the core `sha1_digest_block_u32` function is verified.

## Build

```sh
export SAW_RUST_LIBRARY_PATH="/path/to/mir-json/rlibs"
cargo saw-build
```

## Run proof

```sh
CRYPTOLPATH="../cryptol-specs" saw sha1.saw
```
