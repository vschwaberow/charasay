use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    // Represents an IO error
    IoError(std::io::Error),
    // Represents a UTF-8 decoding error
    Utf8Error(std::str::Utf8Error),
    // Pathbuf error
    InvalidPath(std::path::PathBuf),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Format the error message as "IO error: <error>"
            CustomError::IoError(err) => write!(f, "IO error: {}", err),
            // Format the error message as "UTF-8 error: <error>"
            CustomError::Utf8Error(err) => write!(f, "UTF-8 error: {}", err),
            // Format the error message as "Invalid path: <path>"
            CustomError::InvalidPath(path) => write!(f, "Invalid path: {}", path.display()),
        }
    }
}

impl Error for CustomError {}
