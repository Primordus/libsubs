
use std::env;
use std::fs::File;
use std::io::Write;

use open_subtitles::error::Error as OpenSubtitleError;
use downloader::Download;
use super::format_episode;

/// Representation of 1 subtitle on OpenSubtitles.org
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
        let zip_location = self.tmp_dir() + &format_episode(episode_name) + ".zip";
        let mut file = try!(File::create(&zip_location).map_err(|e| {
            OpenSubtitleError::FileError(format!("Could not create zip file at {}: {}", zip_location, e))
        }));
        try!(file.write_all(zip_bytes.as_bytes()).map_err(|e| {
            OpenSubtitleError::FileError(format!("Could not write to zip file at {}: {}", zip_location, e))
        }));
        Ok(zip_location)
    }

    fn tmp_dir(&self) -> String {
        env::temp_dir().into_os_string().into_string().unwrap()
    }
}
