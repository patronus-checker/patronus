use std::io;

/// Errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Finding providers failed.
    IoError {
        /// The source error.
        source: io::Error,
    },
    /// Loading provider failed.
    LibloadingError {
        /// The source error.
        source: libloading::Error,
    },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;
        match *self {
            IoError { ref source } => Some(source),
            LibloadingError { ref source } => Some(source),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match *self {
            IoError { ref source } => write!(f, "IO failed: {}", source),
            LibloadingError { ref source } => write!(f, "Libloading failed: {}", source),
        }
    }
}
