use sha1::*;

const HELLO: &[u8; 11] = b"hello world";

pub fn example() {
    Sha1::digest(HELLO);

    let mut hasher = Sha1::new();
    hasher.update(HELLO);
    hasher.finalize();
}
