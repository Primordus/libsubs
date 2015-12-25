
extern crate libsubs;

use std::fs;
use std::path::Path;
use libsubs::downloader::Downloader;
use libsubs::fetchable::Fetchable;
use libsubs::open_subtitles::OpenSubtitles;

#[test]
fn integration_test() {
    let dl = Box::new(Downloader::new());
    let os = OpenSubtitles::new(dl);
    let expected_file = "tests/fixtures/file1.srt";
    os.fetch("tests/fixtures/file1.avi", "eng").unwrap();
    assert!(Path::new(expected_file).exists());
    fs::remove_file(expected_file).unwrap();
}

