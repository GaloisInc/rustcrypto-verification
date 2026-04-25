# AES

Verification of encryption and decryption for AES-128, AES-192, and AES-256 from
the [`aes`](https://crates.io/crates/aes/0.8.4) crate against the [AES
specification](../cryptol-specs/Primitive/Symmetric/Cipher/Block/AES/Specification.cry)
from `cryptol-specs`.

Note: the upstream `aes-0.8.4` has a dependency on
[`generic-array`](https://crates.io/crates/generic-array), which uses some
unsafe operations that `crucible-mir` currently cannot simulate. Therefore, we
verify a [fork of
`aes-0.8.4`](https://github.com/RyanGlScott/block-ciphers/tree/backport-hybrid-arrays-to-aes-0.8.4),
which in turn uses a [fork of
`generic-array`](https://github.com/RyanGlScott/generic-array/tree/backport-hybrid-arrays-to-generic-array-0.14.7)
which avoids the unsafe operations. The `generic-array` fork preserves the
behavior of the upstream version, and the `aes` fork is unchanged from upstream
`aes` except for which `generic-array` it depends on; no Rust source code in
`aes` was modified. `generic-array` provides an array data structure, and does
not have to do with cryptography. Hence, no cryptographic code has been modified
between the upstream crates and our forks, and since we have verified our forks
to be correct, we can have high confidence that upstream `aes-0.8.4` is correct
as well.

The top-level Rust functions to be verified are in `src/lib.rs`. The Cryptol
specs are in `AesVerif/`. The SAW proof is in `aes.saw`.

## Build

```sh
export SAW_RUST_LIBRARY_PATH="/path/to/mir-json/rlibs"
RUSTFLAGS="--cfg aes_force_soft" cargo saw-build
```

Make sure the `.linked-mir.json` path in the `linking <n> mir files into`
message from `cargo saw-build` matches the path given to the `mir_load_module`
command in `aes.saw`. If not you will have to update the path in `aes.saw`.

## Run proof

```sh
CRYPTOLPATH="../cryptol-specs" saw aes.saw
```

The proof takes about 19 minutes to run on a M4 Mac.
