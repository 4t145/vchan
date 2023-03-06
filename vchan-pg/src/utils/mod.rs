use sha2::Digest;

pub fn vchan_hash(seed: impl AsRef<[u8]>, name: impl AsRef<[u8]>) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(seed);
    hasher.update(b"==@==");
    hasher.update(name);
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    return output;
}