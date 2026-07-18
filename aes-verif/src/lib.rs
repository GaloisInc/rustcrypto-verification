use aes::{
    Aes128, Aes192, Aes256,
    cipher::{
        BlockDecrypt, BlockEncrypt, KeyInit, generic_array::GenericArray,
    },
};

/// Encrypt a block using AES-128.
pub fn toplevel_encrypt_block_128(key: [u8; 16], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let cipher = Aes128::new(&key);
    let mut block = GenericArray::from(block);
    cipher.encrypt_block(&mut block);
    block.into()
}

/// Decrypt a block using AES-128.
pub fn toplevel_decrypt_block_128(key: [u8; 16], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let cipher = Aes128::new(&key);
    let mut block = GenericArray::from(block);
    cipher.decrypt_block(&mut block);
    block.into()
}

/// Encrypt a block using AES-192.
pub fn toplevel_encrypt_block_192(key: [u8; 24], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let cipher = Aes192::new(&key);
    let mut block = GenericArray::from(block);
    cipher.encrypt_block(&mut block);
    block.into()
}

/// Decrypt a block using AES-192.
pub fn toplevel_decrypt_block_192(key: [u8; 24], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let cipher = Aes192::new(&key);
    let mut block = GenericArray::from(block);
    cipher.decrypt_block(&mut block);
    block.into()
}

/// Encrypt a block using AES-256.
pub fn toplevel_encrypt_block_256(key: [u8; 32], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let cipher = Aes256::new(&key);
    let mut block = GenericArray::from(block);
    cipher.encrypt_block(&mut block);
    block.into()
}

/// Decrypt a block using AES-256.
pub fn toplevel_decrypt_block_256(key: [u8; 32], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let cipher = Aes256::new(&key);
    let mut block = GenericArray::from(block);
    cipher.decrypt_block(&mut block);
    block.into()
}
