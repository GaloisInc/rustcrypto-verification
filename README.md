# rustcrypto-verification

Verification of various [RustCrypto](https://github.com/RustCrypto) crates
against specifications from
[`cryptol-specs`](https://github.com/GaloisInc/cryptol-specs).

## Current status

| Crate | Version | Verified
| --- | --- | --- |
| [`aes`](https://crates.io/crates/aes/0.8.4) | `0.8.4` | AES-128, AES-192, AES-256 |
| [`sha2`](https://crates.io/crates/sha2/0.10.9) | `0.10.9` | SHA-384, SHA-512 |

## Development

Make sure to initialize the `cryptol-specs` submodule after cloning.
