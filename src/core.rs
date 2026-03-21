use std::fs::{File, metadata};
use std::io::{self, Read, Seek};
use std::path::Path;

pub const COMMON_KEY_OFFSET: u64 = 0xE0;
pub const COMMON_KEY_SIZE: usize = 16;
pub const OTP_SIZE: u64 = 1024;

// Changed signature from &str to &Path
pub fn extract_common_key(path: &Path) -> io::Result<[u8; COMMON_KEY_SIZE]> {
    let mut file = File::open(path)?;
    file.seek(io::SeekFrom::Start(COMMON_KEY_OFFSET))?;

    let mut key = [0u8; COMMON_KEY_SIZE];
    file.read_exact(&mut key)?;
    Ok(key)
}

// Changed signature from &str to &Path
pub fn is_valid_otp(path: &Path) -> bool {
    if path.extension().and_then(|s| s.to_str()) != Some("bin") {
        return false;
    }

    // Clippy fix: needless_borrow - `path` is already a &Path
    match metadata(path) {
        Ok(meta) => meta.len() == OTP_SIZE,
        Err(_) => false,
    }
}
