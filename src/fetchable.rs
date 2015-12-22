
use subtitle_error::SubtitleError;

/*
 * Trait which should be implemented for each API you want to fetch information from.
 */
pub trait Fetchable {
    fn fetch(&self, episode_name: &str, language: &str) -> Result<String, SubtitleError>;
}

