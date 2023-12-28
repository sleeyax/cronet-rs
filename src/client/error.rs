use core::fmt;

use crate::CronetError;

pub enum ClientError {
    /// Internal cronet error.
    CronetError(CronetError),
    /// The request was cancelled.
    CancellationError,
}

impl From<CronetError> for ClientError {
    fn from(error: CronetError) -> Self {
        Self::CronetError(error)
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CronetError(error) => write!(f, "{}", error),
            Self::CancellationError => write!(f, "Request was cancelled"),
        }
    }
}

impl fmt::Debug for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CronetError(error) => write!(f, "{:?}", error),
            Self::CancellationError => write!(f, "Request was cancelled"),
        }
    }
}
