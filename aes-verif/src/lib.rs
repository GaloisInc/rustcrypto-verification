use aes::{
    Aes128, Aes192, Aes256,
    cipher::{
        BlockDecrypt, BlockEncrypt, KeyInit, generic_array::GenericArray,
    },
};

/// Encrypt a block using AES-128.
pub fn encrypt128(key: [u8; 16], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let mut block = GenericArray::from(block);
    let cipher = Aes128::new(&key);
    cipher.encrypt_block(&mut block);
    block.into()
}

/// Decrypt a block using AES-128.
pub fn decrypt128(key: [u8; 16], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let mut block = GenericArray::from(block);
    let cipher = Aes128::new(&key);
    cipher.decrypt_block(&mut block);
    block.into()
}

/// Encrypt a block using AES-192.
pub fn encrypt192(key: [u8; 24], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let mut block = GenericArray::from(block);
    let cipher = Aes192::new(&key);
    cipher.encrypt_block(&mut block);
    block.into()
}

/// Decrypt a block using AES-192.
pub fn decrypt192(key: [u8; 24], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let mut block = GenericArray::from(block);
    let cipher = Aes192::new(&key);
    cipher.decrypt_block(&mut block);
    block.into()
}

/// Encrypt a block using AES-256.
pub fn encrypt256(key: [u8; 32], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let mut block = GenericArray::from(block);
    let cipher = Aes256::new(&key);
    cipher.encrypt_block(&mut block);
    block.into()
}

/// Decrypt a block using AES-256.
pub fn decrypt256(key: [u8; 32], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let mut block = GenericArray::from(block);
    let cipher = Aes256::new(&key);
    cipher.decrypt_block(&mut block);
    block.into()
}
