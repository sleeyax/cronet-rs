use std::ffi::CString;

use crate::{
    Buffer, Cronet_UrlRequestPtr, Cronet_UrlRequest_Cancel, Cronet_UrlRequest_Create,
    Cronet_UrlRequest_Destroy, Cronet_UrlRequest_FollowRedirect, Cronet_UrlRequest_GetStatus,
    Cronet_UrlRequest_InitWithParams, Cronet_UrlRequest_IsDone, Cronet_UrlRequest_Read,
    Cronet_UrlRequest_Start, Destroy, Engine, EngineResult, Executor, UrlRequestCallback,
    UrlRequestParams, UrlRequestStatusListener,
};

/// Controls an HTTP request.
pub struct UrlRequest {
    pub(crate) ptr: Cronet_UrlRequestPtr,
}

impl UrlRequest {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: Cronet_UrlRequest_Create(),
            }
        }
    }

    /// Initializes a `URLRequest` with the given `url` and `params`. All methods of the `callback` for
    /// the request will be invoked on the specified `executor`. The `executor` must not run tasks on
    /// the thread calling `Executor::Execute()` to prevent blocking networking
    /// operations and causing failure `RESULT`s during shutdown.
    ///
    /// Arguments:
    ///
    /// * `engine` - Engine to process the request.
    /// * `url` - URL for the request.
    /// * `params` - Additional parameters for the request, like headers and priority.
    /// * `callback` - Callback that gets invoked on different events.
    /// * `executor` - Executor on which all callbacks will be invoked.
    pub fn init_with_params(
        &self,
        engine: Engine,
        url: &str,
        params: UrlRequestParams,
        callback: UrlRequestCallback,
        executor: Executor,
    ) -> EngineResult {
        unsafe {
            let c_str = CString::new(url).unwrap();
            let result = Cronet_UrlRequest_InitWithParams(
                self.ptr,
                engine.ptr,
                c_str.as_ptr(),
                params.ptr,
                callback.ptr,
                executor.ptr,
            );
            EngineResult::try_from(result).unwrap()
        }
    }

    /// Starts the request, and all callbacks go to [crate::UrlRequestCallbackHandler].
    /// This method may only be called once and may not be called if `cancel()` has been called.
    pub fn start(&self) -> EngineResult {
        unsafe {
            let result = Cronet_UrlRequest_Start(self.ptr);
            EngineResult::try_from(result).unwrap()
        }
    }

    /// Follows a pending redirect.
    /// Must only be called at most once for each invocation of `UrlRequestCallbackHandler::OnRedirectReceived()`.
    pub fn follow_redirect(&self) -> EngineResult {
        unsafe {
            let result = Cronet_UrlRequest_FollowRedirect(self.ptr);
            EngineResult::try_from(result).unwrap()
        }
    }

    /// Attempts to read part of the response body into the provided buffer. This method must only be called at most once in response to each invocation of the
    /// `URLRequestCallbackHandler::OnResponseStarted()` and `URLRequestCallbackHandler::OnReadCompleted()` methods of the [crate::UrlRequestCallbackHandler].
    ///
    /// Each call will result in an asynchronous call to either the `URLRequestCallbackHandler::OnReadCompleted()` method if data is read,
    /// its `URLRequestCallbackHandler::OnSucceeded()` method
    /// if there's no more data to read, or its `URLRequestCallbackHandler::OnFailed()` method if there's an error.
    ///
    /// This method transfers ownership of `buffer` to Cronet,
    /// and the app should not read or modify the buffer's position, limit, or data between its position and
    /// limit until the request calls back into the [crate::UrlRequestCallbackHandler].
    ///
    /// Arguments:
    ///
    /// * `buffer`: Buffer to write the response body to.
    /// The app must not read or modify the buffer's position, limit, or data between its position and limit until the request calls back into the `URLRequestCallbackHandler`.
    pub fn read(&self, buffer: Buffer) -> EngineResult {
        unsafe {
            let result = Cronet_UrlRequest_Read(self.ptr, buffer.ptr);
            EngineResult::try_from(result).unwrap()
        }
    }

    /// Cancels the request and can be called at any time.
    ///
    /// When cancellation is complete, `URLRequestCallbackHandler::OnCanceled()` will be invoked, and no further callback methods will be invoked.
    /// If the request has completed or has not started, calling `cancel()` has no effect, and `URLRequestCallbackHandler::OnCanceled()` will not be invoked.
    /// If the [Executor] passed into `UrlRequest::InitWithParams()` runs tasks on a single thread, and `Cancel()` is called on that thread,
    /// no callback methods (besides `URLRequestCallbackHandler::OnCanceled()`) will be invoked after `Cancel()` is called.
    /// Otherwise, at most one callback method may be invoked after `Cancel()` has completed.
    pub fn cancel(&self) {
        unsafe {
            Cronet_UrlRequest_Cancel(self.ptr);
        }
    }

    /// Returns `true` if the request was successfully started and is now finished (completed, canceled, or failed).
    pub fn is_done(&self) -> bool {
        unsafe { Cronet_UrlRequest_IsDone(self.ptr) }
    }

    /// Queries the status of the request.
    ///
    /// Arguments:
    ///
    /// * `listener`: A `URLRequestStatusListener` that will be invoked with the request's current status.
    /// The listener will be invoked back on the [Executor] passed in when the request was created.
    pub fn status(&self, listener: UrlRequestStatusListener) {
        unsafe {
            Cronet_UrlRequest_GetStatus(self.ptr, listener.ptr);
        }
    }
}

impl Destroy for UrlRequest {
    fn destroy(&self) {
        unsafe {
            Cronet_UrlRequest_Destroy(self.ptr);
        }
    }
}
