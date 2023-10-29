#[derive(Debug, PartialEq)]
pub enum EngineResult {
    /// Operation completed successfully
    Success = 0,

    /// Illegal argument
    IllegalArgument = -100,

    /// Storage path must exist
    IllegalArgumentStoragePathMustExist = -101,

    /// Invalid public key pin
    IllegalArgumentInvalidPin = -102,

    /// Invalid hostname
    IllegalArgumentInvalidHostname = -103,

    /// Invalid HTTP method
    IllegalArgumentInvalidHttpMethod = -104,

    /// Invalid HTTP header
    IllegalArgumentInvalidHttpHeader = -105,

    /// Illegal state
    IllegalState = -200,

    /// Storage path in use by another engine
    IllegalStateStoragePathInUse = -201,

    /// Cannot shut down engine from network thread
    IllegalStateCannotShutdownEngineFromNetworkThread = -202,

    /// Engine already started
    IllegalStateEngineAlreadyStarted = -203,

    /// Request already started
    IllegalStateRequestAlreadyStarted = -204,

    /// Request not initialized
    IllegalStateRequestNotInitialized = -205,

    /// Request already initialized
    IllegalStateRequestAlreadyInitialized = -206,

    /// Request not started
    IllegalStateRequestNotStarted = -207,

    /// Unexpected redirect
    IllegalStateUnexpectedRedirect = -208,

    /// Unexpected read attempt
    IllegalStateUnexpectedRead = -209,

    /// Unexpected read failure
    IllegalStateReadFailed = -210,

    /// Null pointer or empty data
    NullPointer = -300,

    /// Hostname cannot be null
    NullPointerHostname = -301,

    /// Set of SHA256 pins cannot be null
    NullPointerSha256Pins = -302,

    /// Pin expiration date cannot be null
    NullPointerExpirationDate = -303,

    /// Engine is required
    NullPointerEngine = -304,

    /// URL is required
    NullPointerURL = -305,

    /// Callback is required
    NullPointerCallback = -306,

    /// Executor is required
    NullPointerExecutor = -307,

    /// Method is required
    NullPointerMethod = -308,

    /// Invalid header name
    NullPointerHeaderName = -309,

    /// Invalid header value
    NullPointerHeaderValue = -310,

    /// Params is required
    NullPointerParams = -311,

    /// Executor for RequestFinishedInfoListener is required
    NullPointerRequestFinishedInfoListenerExecutor = -312,
}

impl TryFrom<i32> for EngineResult {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EngineResult::Success),
            -100 => Ok(EngineResult::IllegalArgument),
            -101 => Ok(EngineResult::IllegalArgumentStoragePathMustExist),
            -102 => Ok(EngineResult::IllegalArgumentInvalidPin),
            -103 => Ok(EngineResult::IllegalArgumentInvalidHostname),
            -104 => Ok(EngineResult::IllegalArgumentInvalidHttpMethod),
            -105 => Ok(EngineResult::IllegalArgumentInvalidHttpHeader),
            -200 => Ok(EngineResult::IllegalState),
            -201 => Ok(EngineResult::IllegalStateStoragePathInUse),
            -202 => Ok(EngineResult::IllegalStateCannotShutdownEngineFromNetworkThread),
            -203 => Ok(EngineResult::IllegalStateEngineAlreadyStarted),
            -204 => Ok(EngineResult::IllegalStateRequestAlreadyStarted),
            -205 => Ok(EngineResult::IllegalStateRequestNotInitialized),
            -206 => Ok(EngineResult::IllegalStateRequestAlreadyInitialized),
            -207 => Ok(EngineResult::IllegalStateRequestNotStarted),
            -208 => Ok(EngineResult::IllegalStateUnexpectedRedirect),
            -209 => Ok(EngineResult::IllegalStateUnexpectedRead),
            -210 => Ok(EngineResult::IllegalStateReadFailed),
            -300 => Ok(EngineResult::NullPointer),
            -301 => Ok(EngineResult::NullPointerHostname),
            -302 => Ok(EngineResult::NullPointerSha256Pins),
            -303 => Ok(EngineResult::NullPointerExpirationDate),
            -304 => Ok(EngineResult::NullPointerEngine),
            -305 => Ok(EngineResult::NullPointerURL),
            -306 => Ok(EngineResult::NullPointerCallback),
            -307 => Ok(EngineResult::NullPointerExecutor),
            -308 => Ok(EngineResult::NullPointerMethod),
            -309 => Ok(EngineResult::NullPointerHeaderName),
            -310 => Ok(EngineResult::NullPointerHeaderValue),
            -311 => Ok(EngineResult::NullPointerParams),
            -312 => Ok(EngineResult::NullPointerRequestFinishedInfoListenerExecutor),
            _ => Err(()),
        }
    }
}
