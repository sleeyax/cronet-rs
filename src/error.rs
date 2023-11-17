use std::{
    ffi::{CStr, CString},
    fmt,
};

use crate::{
    Cronet_ErrorPtr, Cronet_Error_Create, Cronet_Error_Destroy, Cronet_Error_error_code_get,
    Cronet_Error_error_code_set, Cronet_Error_immediately_retryable_get,
    Cronet_Error_immediately_retryable_set, Cronet_Error_internal_error_code_get,
    Cronet_Error_internal_error_code_set, Cronet_Error_message_get, Cronet_Error_message_set,
    Cronet_Error_quic_detailed_error_code_get, Cronet_Error_quic_detailed_error_code_set, Destroy,
};

#[derive(Debug)]
pub struct CronetError {
    pub(crate) ptr: Cronet_ErrorPtr,
}

impl CronetError {
    pub fn new() -> Self {
        unsafe {
            CronetError {
                ptr: Cronet_Error_Create(),
            }
        }
    }

    /// Set the [ErrorCode].
    pub fn set_error_code(&self, error_code: ErrorCode) {
        unsafe {
            Cronet_Error_error_code_set(self.ptr, error_code as u32);
        }
    }

    /// Get the [ErrorCode].
    pub fn error_code(&self) -> ErrorCode {
        unsafe {
            let code = Cronet_Error_error_code_get(self.ptr);
            code.try_into().unwrap()
        }
    }

    /// Set the error message.
    pub fn set_message(&self, message: &str) {
        unsafe {
            let c_message = CString::new(message).unwrap();
            Cronet_Error_message_set(self.ptr, c_message.as_ptr());
        }
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        unsafe {
            let c_message = Cronet_Error_message_get(self.ptr);
            let message = CStr::from_ptr(c_message).to_str().unwrap();
            message
        }
    }

    /// Set the internal error code.
    ///
    /// See [internal_error_code](#method.internal_error_code) for more information.
    pub fn set_internal_error_code(&self, internal_error_code: i32) {
        unsafe { Cronet_Error_internal_error_code_set(self.ptr, internal_error_code) }
    }

    /// Get the internal error code.
    /// This may provide more specific error diagnosis than [ErrorCode], but the constant values may change over time.
    ///
    /// See <a href=https://chromium.googlesource.com/chromium/src/+/main/net/base/net_error_list.h> here</a> for the latest list of values.
    pub fn internal_error_code(&self) -> i32 {
        unsafe { Cronet_Error_internal_error_code_get(self.ptr) }
    }

    /// Set whether the error is immediately retryable.
    ///
    /// See [retryable](#method.retryable) for more information.
    pub fn set_retryable(&self, retryable: bool) {
        unsafe {
            Cronet_Error_immediately_retryable_set(self.ptr, retryable);
        }
    }

    /// Whether the error is immediately retryable.
    ///
    /// Returns `true` if retrying this request right away might succeed, `false` otherwise.
    ///
    /// For example, it's `true` when [ErrorCode] is [ErrorCode::NetworkChanged]
    /// because trying the request might succeed using the new
    /// network configuration, but `false` when [ErrorCode] is [ErrorCode::InternetDisconnected] because retrying the request right away will
    /// encounter the same failure (instead retrying should be delayed until device regains network connectivity).
    pub fn retryable(&self) -> bool {
        unsafe { Cronet_Error_immediately_retryable_get(self.ptr) }
    }

    /// Set the detailed <a href="https://www.chromium.org/quic">QUIC</a> error code.
    ///
    /// See [quic_detailed_error_code](#method.quic_detailed_error_code) for more information.
    pub fn set_quic_detailed_error_code(&self, quic_detailed_error_code: i32) {
        unsafe { Cronet_Error_quic_detailed_error_code_set(self.ptr, quic_detailed_error_code) }
    }

    /// Contains detailed <a href="https://www.chromium.org/quic">QUIC</a> error code
    /// from <a href="https://cs.chromium.org/search/?q=symbol:%5CbQuicErrorCode%5Cb">QuicErrorCode</a>
    /// when the [ErrorCode] code is [ErrorCode::QuicProtocolFailed].
    pub fn quic_detailed_error_code(&self) -> i32 {
        unsafe { Cronet_Error_quic_detailed_error_code_get(self.ptr) }
    }
}

impl Destroy for CronetError {
    fn destroy(&self) {
        unsafe {
            Cronet_Error_Destroy(self.ptr);
        }
    }
}

impl fmt::Display for CronetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CronetError {{ code: {:?}, message: {:?} }}",
            self.error_code(),
            self.message()
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    /// Error returned by app callback.
    Callback = 0,

    /// Host being sent the request could not be resolved to an IP address.
    HostnameNotResolved = 1,

    /// Device was not connected to any network.
    InternetDisconnected = 2,

    /// Network configuration changed as the request was processed.
    NetworkChanged = 3,

    /// Timeout expired.
    /// Timeouts expiring while attempting to connect will be reported as the more specific `ConnectionTimedOut`.
    TimedOut = 4,

    /// Connection was closed unexpectedly.
    ConnectionClosed = 5,

    /// Connection attempt timed out.
    ConnectionTimedOut = 6,

    /// Connection attempt was refused.
    ConnectionRefused = 7,

    /// Connection was unexpectedly reset.
    ConnectionReset = 8,

    /// IP address being contacted is unreachable, meaning there is no route to the specified host or network.
    AddressUnreachable = 9,

    /// Error related to the QUIC protocol.
    /// When the code is this, see `QuicDetailedCode` for more information.
    QuicProtocolFailed = 10,

    /// Another type of error was encountered.
    /// Consult `InternalCode` to get a more specific cause.
    Other = 11,
}

impl TryFrom<u32> for ErrorCode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ErrorCode::Callback),
            1 => Ok(ErrorCode::HostnameNotResolved),
            2 => Ok(ErrorCode::InternetDisconnected),
            3 => Ok(ErrorCode::NetworkChanged),
            4 => Ok(ErrorCode::TimedOut),
            5 => Ok(ErrorCode::ConnectionClosed),
            6 => Ok(ErrorCode::ConnectionTimedOut),
            7 => Ok(ErrorCode::ConnectionRefused),
            8 => Ok(ErrorCode::ConnectionReset),
            9 => Ok(ErrorCode::AddressUnreachable),
            10 => Ok(ErrorCode::QuicProtocolFailed),
            11 => Ok(ErrorCode::Other),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Destroy;

    #[test]
    fn test_cronet_error() {
        let cronet_error = super::CronetError::new();
        cronet_error.set_error_code(super::ErrorCode::TimedOut);
        cronet_error.set_message("test");
        cronet_error.set_internal_error_code(1337);
        cronet_error.set_retryable(true);
        cronet_error.set_quic_detailed_error_code(10);
        assert_eq!(cronet_error.error_code(), super::ErrorCode::TimedOut);
        assert_eq!(cronet_error.message(), "test");
        assert_eq!(cronet_error.internal_error_code(), 1337);
        assert_eq!(cronet_error.retryable(), true);
        assert_eq!(cronet_error.quic_detailed_error_code(), 10);
        cronet_error.destroy();
    }
}
