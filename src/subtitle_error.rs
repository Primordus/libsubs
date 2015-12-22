
use std::error;
use std::fmt;

use open_subtitles::error::Error as OpenSubError;

#[derive(Debug)]
pub enum SubtitleError {
    /// Errors related to opensubtitles.org
    OpenSubtitleError(OpenSubError),

    // Possibly errors from other APIs can be added here.
}

impl fmt::Display for SubtitleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubtitleError::OpenSubtitleError(ref e) => e.fmt(f)
        }
    }
}

impl error::Error for SubtitleError {
    fn description(&self) -> &str {
        match *self {
            SubtitleError::OpenSubtitleError(ref e) => e.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SubtitleError::OpenSubtitleError(ref e) => Some(e)
        }
    }
}

impl From<OpenSubError> for SubtitleError {
    fn from(error: OpenSubError) -> SubtitleError {
        SubtitleError::OpenSubtitleError(error)
    }
}

