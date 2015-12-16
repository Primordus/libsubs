pub mod movie_hash {
    
    use libc;
    use std::ffi::CString;

    // This has to be inside the module, otherwise not found!
    // And error messages don't really help..

    #[allow(dead_code)]
    #[repr(C)]
    enum ResultType {
        OK,
        FILE_NOT_FOUND
    }

    #[repr(C)]
    struct HashResult {
        result_type: ResultType,
        hash: libc::uint64_t
    }

    #[link(name = "movie_hash", kind = "static")]
    extern {
        fn calc_hash(episode_name: *const libc::c_char) -> HashResult;
    }

    pub fn compute_hash(episode: &'static str) -> Result<u64, &'static str> {
        let episode_name = CString::new(episode).unwrap();
        unsafe {
            match calc_hash(episode_name.as_ptr()) {
                HashResult { result_type: ResultType::OK, hash } => Ok(hash),
                HashResult { result_type: ResultType::FILE_NOT_FOUND, .. } => Err("File not found!")
            }
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

        assert!(compute_hash("unknown_file") == Err("File not found!"));
        assert!(compute_hash("./tests/fixtures/file1.avi") == Ok(0x8e245d9679d31e12));
        assert!(compute_hash("./tests/fixtures/file2.bin") == Ok(0x61f7751fc2a72bfb));
    }
}
