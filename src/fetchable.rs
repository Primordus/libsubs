
// TODO create separate subtitle error per API which can be converted back to this error..
#[derive(Debug, PartialEq)]
pub enum SubtitleError {
    HashError,
    DownloadError(String),
    XmlError,
    ParseError,
    FileError(String),
    NoSubtitlesFound,
    UnzipError
}

/*
 * Trait which should be implemented for each API you want to fetch information from.
 */
pub trait Fetchable {
    fn fetch(&self, episode_name: &str, language: &str) -> Result<String, SubtitleError>;
}

