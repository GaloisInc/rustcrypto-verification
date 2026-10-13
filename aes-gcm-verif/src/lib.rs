//! This module instantiates the [AesGcm::new], [AesGcm::encrypt], and
//! [AesGcm::decrypt] functions with the parameters that are checked in
//! `aes-gcm.saw`. While the plaintext/ciphertext and AAD lengths are not
//! statically tracked (because they are slices), the AES key size, nonce size,
//! and tag size are all encoded in the instantiated [AesGcm] type. So the
//! resulting MIR module must have instantiations of [AesGcm] with the cartesian
//! product of all these sizes. We do this by having a series of functions where
//! each one progressively instantiates one more dimension of the product.

use aes_gcm::{
    AesGcm, Key, KeyInit, Nonce,
    aead::{
        Aead, Payload,
        consts::{U1, U8, U12, U13, U14, U15, U16},
    },
    aes::{
        Aes128, Aes192, Aes256,
        cipher::{ArrayLength, BlockCipher, BlockEncrypt, BlockSizeUser},
    },
};

pub fn inst0<Aes, NonceSize, TagSize>(
    key: &Key<Aes>,
    nonce: &Nonce<NonceSize>,
    msg: &[u8],
    aad: &[u8],
) where
    Aes: BlockCipher + BlockSizeUser<BlockSize = U16> + BlockEncrypt + KeyInit,
    NonceSize: ArrayLength<u8>,
    TagSize: aes_gcm::TagSize,
{
    let cipher = AesGcm::<Aes, NonceSize, TagSize>::new(key);
    let _ = cipher.encrypt(nonce, Payload { msg, aad });
    let _ = cipher.decrypt(nonce, Payload { msg, aad });
}

pub fn inst1<NonceSize, TagSize>(
    keys: &(Key<Aes128>, Key<Aes192>, Key<Aes256>),
    nonce: &Nonce<NonceSize>,
    msg: &[u8],
    aad: &[u8],
) where
    NonceSize: ArrayLength<u8>,
    TagSize: aes_gcm::TagSize,
{
    inst0::<Aes128, NonceSize, TagSize>(&keys.0, nonce, msg, aad);
    inst0::<Aes192, NonceSize, TagSize>(&keys.1, nonce, msg, aad);
    inst0::<Aes256, NonceSize, TagSize>(&keys.2, nonce, msg, aad);
}

pub fn inst2<TagSize>(
    keys: &(Key<Aes128>, Key<Aes192>, Key<Aes256>),
    nonces: &(Nonce<U1>, Nonce<U8>, Nonce<U12>, Nonce<U16>),
    msg: &[u8],
    aad: &[u8],
) where
    TagSize: aes_gcm::TagSize,
{
    inst1::<U1, TagSize>(keys, &nonces.0, msg, aad);
    inst1::<U8, TagSize>(keys, &nonces.1, msg, aad);
    inst1::<U12, TagSize>(keys, &nonces.2, msg, aad);
    inst1::<U16, TagSize>(keys, &nonces.3, msg, aad);
}

pub fn inst3(
    keys: &(Key<Aes128>, Key<Aes192>, Key<Aes256>),
    nonces: &(Nonce<U1>, Nonce<U8>, Nonce<U12>, Nonce<U16>),
    msg: &[u8],
    aad: &[u8],
) {
    inst2::<U12>(keys, nonces, msg, aad);
    inst2::<U13>(keys, nonces, msg, aad);
    inst2::<U14>(keys, nonces, msg, aad);
    inst2::<U15>(keys, nonces, msg, aad);
    inst2::<U16>(keys, nonces, msg, aad);
}
