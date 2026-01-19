use aes::{
    Aes128,
    cipher::{Array, BlockCipherEncrypt, KeyInit},
};

pub fn encrypt128(key: [u8; 16], in_block: [u8; 16]) -> [u8; 16] {
    let key = Array(key);
    let in_block = Array(in_block);
    let cipher = Aes128::new(&key);
    let mut out_block = Array([0; 16]);
    cipher.encrypt_block_b2b(&in_block, &mut out_block);
    out_block.0
}
