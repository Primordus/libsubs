
use std::error;
use std::fmt;
use std::convert::From;

use xml_parser::XmlError as XmlErr;
use open_subtitles::movie_hash;

// TODO replace string with an actual error type?
#[derive(Debug, PartialEq)]
pub enum Error {
    NoSubtitlesFound,
    HashError(movie_hash::HashError),
    XmlError(XmlErr),
    DownloadError(String),
    FileError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HashError(ref reason) => {
                write!(f, "Problem calculating subtitle hash: {}", reason)
            }
            Error::DownloadError(ref reason) => {
                write!(f, "Error during download: {}", reason)
            }
            Error::XmlError(ref reason) => {
                write!(f, "Error while handling XML: {}", reason)
            }
            Error::FileError(ref reason) => {
                write!(f, "File error: {}", reason)
            }
            Error::NoSubtitlesFound => {
                write!(f, "No matching subtitles found")
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HashError(ref reason) => {
                reason.description()
            }
            Error::XmlError(ref reason) => {
                reason.description()
            }
            Error::DownloadError(ref reason) => {
                &reason
            }
            Error::FileError(ref reason) => {
                &reason
            }
            Error::NoSubtitlesFound => {
                "No matching subtitles found"
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::HashError(ref err) => {
                Some(err)
            }
            Error::XmlError(ref err) => {
                Some(err)
            }
            _ => {
                None
            }
        }
    }
}

impl From<movie_hash::HashError> for Error {
    fn from(e: movie_hash::HashError) -> Error {
        Error::HashError(e)
    }    
}

impl From<XmlErr> for Error {
    fn from(e: XmlErr) -> Error {
        Error::XmlError(e)
    }
}

