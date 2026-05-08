use core::fmt;

pub enum Error {
    Http(String),
    InvalidUrl,
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(err) => write!(f, "HTTP error: {}", err),
            Error::InvalidUrl => write!(f, "Invalid URL (expected https?://odysee\\.com/@[^:]+:\\d+.*)"),
            Error::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Error::Http(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Http(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
