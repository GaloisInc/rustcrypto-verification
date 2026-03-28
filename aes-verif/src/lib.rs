use aes::{
    Aes128,
    cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray},
};

pub fn encrypt128(key: [u8; 16], in_block: [u8; 16]) -> [u8; 16] {
    let key = GenericArray::from(key);
    let mut in_block = GenericArray::from(in_block);
    let cipher = Aes128::new(&key);
    cipher.encrypt_block(&mut in_block);
    in_block.into()
}
