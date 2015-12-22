
use libc;
use libc::{c_char, c_void};
use std::ffi::CString;

use std::fmt;
use std::error;

#[link(name = "movie_hash", kind = "static")]
extern {
    fn calculate_hash(episode_name: *const c_char) -> *mut c_void;
    fn destroy_hash(hash: *mut c_void);
    fn is_valid_hash(hash: *const c_void) -> bool;
    fn get_hash(hash: *const c_void) -> u64;
}

pub fn compute_hash(episode: &str) -> Result<u64, HashError> {
    Hash::new(episode).get()
}

struct Hash {
    ptr: *mut libc::c_void  // raw void ptr.
}

impl Hash {
    fn new(episode: &str) -> Hash {
        let episode_name = CString::new(episode).unwrap();
        Hash {
            ptr: unsafe { calculate_hash(episode_name.as_ptr()) }
        }
    }

    fn get(&self) -> Result<u64, HashError> {
        unsafe {
            if is_valid_hash(self.ptr) {
                return Ok(get_hash(self.ptr));
            }
        }

        Err(HashError::InvalidHash)
    }
}

impl Drop for Hash {
    fn drop(&mut self) {
        unsafe { destroy_hash(self.ptr); }
    }
}

#[derive(Debug, PartialEq)]
pub enum HashError {
    InvalidHash
}

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error calculating hash")
    }
}

impl error::Error for HashError {
    fn description(&self) -> &str {
        "Invalid subtitle hash"
    }
}


// Unit tests:

#[test]
fn hashing_test() {
    use std::process::Command; // TODO convert this to pure rust..
    Command::new("./tests/fixtures/download_test_files.sh")
        .status()
        .unwrap_or_else(|e| {
        panic!("Failed to run download script: {}!", e);
    });

    assert!(compute_hash("unknown_file").is_err());
    assert_eq!(compute_hash("./tests/fixtures/file1.avi").unwrap(), 0x8e245d9679d31e12);
    assert_eq!(compute_hash("./tests/fixtures/file2.bin").unwrap(), 0x61f7751fc2a72bfb);
}
