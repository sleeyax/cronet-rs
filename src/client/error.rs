use core::fmt;

use crate::{CronetError, EngineResult};

pub enum ClientError {
    /// Internal cronet error.
    CronetError(CronetError),
    /// The request was cancelled.
    CancellationError,
    /// Unexpected cronet engine result.
    EngineError(EngineResult),
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
            Self::EngineError(error) => write!(f, "Unexpected engine result: {:?}", error),
        }
    }
}

impl fmt::Debug for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CronetError(error) => write!(f, "{:?}", error),
            Self::CancellationError => write!(f, "Request was cancelled"),
            Self::EngineError(error) => write!(f, "Unexpected engine result: {:?}", error),
        }
    }
}
