# AES

Verification of the software implementations of encryption and decryption for
AES-128, AES-192, and AES-256 from the
[`aes`](https://crates.io/crates/aes/0.8.4) crate against the [AES
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

The Cryptol specs and SAW proofs in this directory are also used as part of the
verification of the `aes-gcm` library. Therefore, the SAW code is divided into a
"library" file `aes-lib.saw` and a "driver" file `aes-run.saw`. All the real
verification code is in `aes-lib.saw`, structured in a modular way that works
with any MIR file containing the `aes` crate. `aes-run.saw` then includes
`aes-lib.saw` and uses it to verify the top-level Rust functions defined in
`src/lib.rs`. The Cryptol specs are in `AesVerif/`.

## Build

```sh
export SAW_RUST_LIBRARY_PATH="/path/to/mir-json/rlibs"
RUSTFLAGS="--cfg aes_force_soft" cargo saw-build
```

## Run proof

```sh
CRYPTOLPATH="../cryptol-specs" saw aes.saw
```

The proof takes about 19 minutes to run on a M4 Mac.
