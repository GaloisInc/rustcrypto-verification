# SHA-1

Verification of SHA-1 from the [`sha1`](https://docs.rs/sha1/0.10.6/sha1/) crate
against the [SHA-1
specification](../cryptol-specs/Primitive/Keyless/Hash/SHA1/Specification.cry)
from `cryptol-specs`.

Currently only the core `sha1_digest_block_u32` function is verified.

## Build

```sh
export SAW_RUST_LIBRARY_PATH="/path/to/mir-json/rlibs"
cargo saw-build
```

Make sure the `.linked-mir.json` path in the `linking <n> mir files into`
message from `cargo saw-build` matches the path given to the `mir_load_module`
command in `sha1.saw`. If not you will have to update the path in `sha1.saw`.

## Run proof

```sh
CRYPTOLPATH="../cryptol-specs" saw sha1.saw
```
