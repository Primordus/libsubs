
use libc;
use libc::{c_char, c_void};
use std::ffi::CString;

#[link(name = "movie_hash", kind = "static")]
extern {
    fn calculate_hash(episode_name: *const c_char) -> *mut c_void;
    fn destroy_hash(hash: *mut c_void);
    fn is_valid_hash(hash: *const c_void) -> bool;
    fn get_hash(hash: *const c_void) -> u64;
}

pub fn compute_hash(episode: &'static str) -> Result<u64, &'static str> {
    Hash::new(episode).get()
}

struct Hash {
    ptr: *mut libc::c_void  // raw void ptr.
}

impl Hash {

    fn new(episode: &'static str) -> Hash {
        let episode_name = CString::new(episode).unwrap();
        Hash {
            ptr: unsafe { calculate_hash(episode_name.as_ptr()) }
        }
    }

    fn get(&self) -> Result<u64, &'static str> {
        unsafe {
            if is_valid_hash(self.ptr) {
                return Ok(get_hash(self.ptr));
            }
        }

        // TODO make error more descriptive!
        Err("Problem calculating hash!")
    }
}

impl Drop for Hash {
    fn drop(&mut self) {
        unsafe { destroy_hash(self.ptr); }
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

    assert_eq!(compute_hash("unknown_file"), Err("Problem calculating hash!"));
    assert_eq!(compute_hash("./tests/fixtures/file1.avi"), Ok(0x8e245d9679d31e12));
    assert_eq!(compute_hash("./tests/fixtures/file2.bin"), Ok(0x61f7751fc2a72bfb));
}
