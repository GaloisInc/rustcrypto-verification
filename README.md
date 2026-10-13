# rustcrypto-verification

Verification of various [RustCrypto](https://github.com/RustCrypto) crates
against specifications from
[`cryptol-specs`](https://github.com/GaloisInc/cryptol-specs).

## Current status

| Crate | Version | Verified
| --- | --- | --- |
| [`aes`](https://crates.io/crates/aes/0.8.4) | `0.8.4` | 64-bit software implementation |
| [`aes-gcm`](https://crates.io/crates/aes-gcm/0.10.3) | `0.10.3` | Yes |
| [`sha2`](https://crates.io/crates/sha2/0.10.9) | `0.10.9` | Software implementation of SHA-384 and SHA-512 |
| [`sha1`](https://crates.io/crates/sha1/0.10.6) | `0.10.6` | `sha1_digest_block_u32` function in software implementation |

## Development

Make sure to initialize the `cryptol-specs` submodule after cloning.
