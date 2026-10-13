# AES-GCM

Verification of the [`aes-gcm`](https://crates.io/crates/aes-gcm/0.10.3) crate
against the [GCM
specification](../cryptol-specs/Primitive/Symmetric/Cipher/Authenticated/GCM/Specification.cry)
from `cryptol-specs`.

This depends on the Cryptol specs and SAW script from the [`aes` crate
verification](../aes-verif/).

The SAW proof is in `aes-gcm.saw`. It depends on some Cryptol specs located
under `AesGcmVerif/`.
