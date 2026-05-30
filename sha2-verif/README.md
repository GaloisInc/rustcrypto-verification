# SHA2

Verification of SHA-512 and SHA-384 from the
[`sha2`](https://crates.io/crates/sha2/0.10.9) crate against the [`SHA2Internal`
specification](../cryptol-specs/Primitive/Keyless/Hash/SHA2Internal/SHA.cry)
from `cryptol-specs`.

Note: the upstream `sha2-0.10.9` crate, as well as its dependencies
[`generic-array`](https://crates.io/crates/generic-array) and
[`block-buffer`](https://crates.io/crates/block-buffer), uses some low-level
operations that `crucible-mir` currently cannot handle. Therefore, we verify a
[fork of
`sha2`](https://github.com/RyanGlScott/hashes/tree/crucible-mir-patches/sha2),
which in turn uses a [fork of
`generic-array`](https://github.com/RyanGlScott/generic-array/tree/backport-hybrid-arrays-to-generic-array-0.14.7)
and a [fork of
`block-buffer`](https://github.com/RyanGlScott/utils/tree/crucible-mir-patches/block-buffer),
which avoids the unsupported operations. The patches are minimal and do not
change the behavior of the code (other than possibly causing more allocations).
They also do not touch the core cryptographic part of the code. Hence, since we
have verified our forks to be correct, we can be fairly confident that the
upstream `sha2-0.10.9` is correct as well.

The SAW proof is in `sha2.saw`. It depends on Cryptol specs for Rust functions
in the `sha2` crate, defined in `Sha2Verif/Rust.cry`, as well as an
instantiation of `cryptol-specs`'s `SHA2Internal` for SHA-512, defined in
`SHA2Verif/Spec/SHA512.cry` (`cryptol-specs` only instantiates `SHA2Internal`
for SHA-384 and SHA-256). `src/lib.rs` contains calls to the `sha2` library
functions we want to verify, as well as sample inputs of the input lengths that
we are verifying.

## Build

```sh
export SAW_RUST_LIBRARY_PATH="/path/to/mir-json/rlibs"
cargo saw-build
```

Make sure the `.linked-mir.json` path in the `linking <n> mir files into`
message from `cargo saw-build` matches the path given to the `mir_load_module`
command in `sha2.saw`. If not you will have to update the path in `sha2.saw`.

## Run proof

```sh
CRYPTOLPATH="../cryptol-specs" saw sha2.saw
```

The proof takes about 9 minutes to run on a M4 Mac.
