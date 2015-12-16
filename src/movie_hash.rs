pub mod movie_hash {
    
    use libc;
    use std::ffi::CString;

    // This has to be inside the module, otherwise not found!
    // And error messages don't really help..

    #[allow(dead_code)]
    #[repr(C)]
    enum ResultType {
        OK,
        ERROR
    }

    #[repr(C)]
    struct HashResult {
        result_type: ResultType,
        hash: libc::int64_t
    }

    #[link(name = "movie_hash", kind = "static")]
    extern {
        fn calc_hash(episode_name: *const libc::c_char) -> HashResult;
    }

    pub fn compute_hash(episode: &'static str) -> i64 {  // TODO return Option
        let episode_name = CString::new(episode).unwrap();
        unsafe {
            match calc_hash(episode_name.as_ptr()) {
                HashResult { result_type: ResultType::ERROR, .. } => 0,
                HashResult { result_type: ResultType::OK, hash } => hash
            }
        }
    }

    #[test]
    fn hashing_test() {
        assert!(compute_hash("unknown_file") == 0);
        assert!(compute_hash("Cargo.toml") != 0);
        // TODO more / better tests with test data...
    }
}
