use sha2::*;

/// This function isn't particularly interesting. It only exists to make
/// `cargo saw-build` keep the relevant bits of `sha2`'s API in the generated
/// MIR JSON file.
pub fn example() {
    Sha384::digest(b"hello world");

    let mut hasher = Sha384::new();
    hasher.update(b"hello world");
    hasher.finalize();
}
