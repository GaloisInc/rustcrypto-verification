use sha2::*;

const HELLO: &[u8; 11] = b"hello world";
const LONG_MESSAGE: &[u8; 335] = b"This is a longer message and its length is over two SHA-512 blocks long, where each block is 128 bytes. This allows us to test the case where multiple calls to compress are required during the update phase. Depending on how much data is already in the buffer, calling update() on this should result in either two or three compressions.";

/// This function isn't particularly interesting. It only exists to make
/// `cargo saw-build` keep the relevant bits of `sha2`'s API in the generated
/// MIR JSON file.
pub fn example() {
    Sha384::digest(HELLO);
    Sha384::digest(LONG_MESSAGE);

    Sha512::digest(HELLO);
    Sha512::digest(LONG_MESSAGE);

    let mut hasher384 = Sha384::new();
    hasher384.update(HELLO);
    hasher384.update(LONG_MESSAGE);
    hasher384.finalize();

    let mut hasher512 = Sha512::new();
    hasher512.update(HELLO);
    hasher512.update(LONG_MESSAGE);
    hasher512.finalize();
}
