use aes::{
    Aes128,
    cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray},
};

pub fn encrypt128(key: [u8; 16], block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let mut block = GenericArray::from(block);
    let cipher = Aes128::new(&key);
    cipher.encrypt_block(&mut block);
    block.into()
}
