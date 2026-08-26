use aes_gcm::{
    Aes128Gcm, KeyInit,
    aead::{Aead, Payload},
};

pub fn encrypt128(
    key: [u8; 16],
    iv: [u8; 12],
    msg: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let cipher = Aes128Gcm::new(&key.into());
    cipher.encrypt(&iv.into(), Payload { msg, aad })
}

pub fn decrypt128(
    key: [u8; 16],
    iv: [u8; 12],
    msg: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let cipher = Aes128Gcm::new(&key.into());
    cipher.decrypt(&iv.into(), Payload { msg, aad })
}
