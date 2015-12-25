
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use open_subtitles::error::Error as OpenSubtitleError;
use downloader::Download;

/// Representation of 1 subtitle on OpenSubtitles.org
#[derive(Debug)]
pub struct Subtitle {
    pub url: String,
    pub rating: f64
}

impl Subtitle {
    pub fn new(url: String, rating: f64) -> Subtitle {
        assert!(!rating.is_nan());
        Subtitle {
            url: url,
            rating: rating
        }
    }

    pub fn download(&self, dl: &Box<Download>, episode_name: &str) -> Result<String, OpenSubtitleError> {
        let zip_bytes = try!(dl.download(&self.url).map_err(|e| OpenSubtitleError::DownloadError(e)));
        let file_name = Path::new(episode_name)
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        let zip_location = self.tmp_dir() + &file_name + ".zip";
        let mut file = try!(File::create(&zip_location).map_err(|e| {
            OpenSubtitleError::FileError(format!("Could not create zip file at {}: {}", zip_location, e))
        }));
        try!(file.write_all(&zip_bytes).map_err(|e| {
            OpenSubtitleError::FileError(format!("Could not write to zip file at {}: {}", zip_location, e))
        }));
        Ok(zip_location)
    }

    fn tmp_dir(&self) -> String {
        env::temp_dir().into_os_string().into_string().unwrap()
    }
}
