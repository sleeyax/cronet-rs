use std::ffi::CStr;

use once_cell::sync::Lazy;

use crate::{
    state::CronetCallbacks, Buffer, CronetError, Cronet_BufferPtr, Cronet_ErrorPtr, Cronet_String,
    Cronet_UrlRequestCallbackPtr, Cronet_UrlRequestCallback_CreateWith,
    Cronet_UrlRequestCallback_Destroy, Cronet_UrlRequestPtr, Cronet_UrlResponseInfoPtr, Destroy,
    UrlRequest, UrlResponseInfo,
};

static mut URL_REQUEST_CALLBACK_CALLBACKS: Lazy<
    CronetCallbacks<Cronet_UrlRequestCallbackPtr, Box<dyn UrlRequestCallbackHandler>>,
> = Lazy::new(CronetCallbacks::new);

#[no_mangle]
unsafe extern "C" fn cronetUrlRequestCallbackOnRedirectReceived(
    self_ptr: Cronet_UrlRequestCallbackPtr,
    request_ptr: Cronet_UrlRequestPtr,
    info_ptr: Cronet_UrlResponseInfoPtr,
    new_location_url: Cronet_String,
) {
    let mut lockedMap = URL_REQUEST_CALLBACK_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get_mut(&self_ptr).unwrap();
    let c_str = CStr::from_ptr(new_location_url);
    let new_location_url = c_str.to_str().unwrap();
    callback.on_redirect_received(
        UrlRequestCallback { ptr: self_ptr },
        UrlRequest { ptr: request_ptr },
        UrlResponseInfo { ptr: info_ptr },
        new_location_url,
    );
}

#[no_mangle]
unsafe extern "C" fn cronetUrlRequestCallbackOnResponseStarted(
    self_ptr: Cronet_UrlRequestCallbackPtr,
    request_ptr: Cronet_UrlRequestPtr,
    info_ptr: Cronet_UrlResponseInfoPtr,
) {
    let mut lockedMap = URL_REQUEST_CALLBACK_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get_mut(&self_ptr).unwrap();
    callback.on_response_started(
        UrlRequestCallback { ptr: self_ptr },
        UrlRequest { ptr: request_ptr },
        UrlResponseInfo { ptr: info_ptr },
    );
}

#[no_mangle]
unsafe extern "C" fn cronetUrlRequestCallbackOnReadCompleted(
    self_ptr: Cronet_UrlRequestCallbackPtr,
    request_ptr: Cronet_UrlRequestPtr,
    info_ptr: Cronet_UrlResponseInfoPtr,
    buffer_ptr: Cronet_BufferPtr,
    bytes_read: u64,
) {
    let mut lockedMap = URL_REQUEST_CALLBACK_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get_mut(&self_ptr).unwrap();
    callback.on_read_completed(
        UrlRequestCallback { ptr: self_ptr },
        UrlRequest { ptr: request_ptr },
        UrlResponseInfo { ptr: info_ptr },
        Buffer { ptr: buffer_ptr },
        bytes_read,
    );
}

#[no_mangle]
unsafe extern "C" fn cronetUrlRequestCallbackOnSucceeded(
    self_ptr: Cronet_UrlRequestCallbackPtr,
    request_ptr: Cronet_UrlRequestPtr,
    info_ptr: Cronet_UrlResponseInfoPtr,
) {
    let mut lockedMap = URL_REQUEST_CALLBACK_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get_mut(&self_ptr).unwrap();
    callback.on_succeeded(
        UrlRequestCallback { ptr: self_ptr },
        UrlRequest { ptr: request_ptr },
        UrlResponseInfo { ptr: info_ptr },
    );
}

#[no_mangle]
unsafe extern "C" fn cronetUrlRequestCallbackOnFailed(
    self_ptr: Cronet_UrlRequestCallbackPtr,
    request_ptr: Cronet_UrlRequestPtr,
    info_ptr: Cronet_UrlResponseInfoPtr,
    error_ptr: Cronet_ErrorPtr,
) {
    let mut lockedMap = URL_REQUEST_CALLBACK_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get_mut(&self_ptr).unwrap();
    callback.on_failed(
        UrlRequestCallback { ptr: self_ptr },
        UrlRequest { ptr: request_ptr },
        UrlResponseInfo { ptr: info_ptr },
        CronetError { ptr: error_ptr },
    );
}

#[no_mangle]
unsafe extern "C" fn cronetUrlRequestCallbackOnCanceled(
    self_ptr: Cronet_UrlRequestCallbackPtr,
    request_ptr: Cronet_UrlRequestPtr,
    info_ptr: Cronet_UrlResponseInfoPtr,
) {
    let mut lockedMap = URL_REQUEST_CALLBACK_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get_mut(&self_ptr).unwrap();
    callback.on_canceled(
        UrlRequestCallback { ptr: self_ptr },
        UrlRequest { ptr: request_ptr },
        UrlResponseInfo { ptr: info_ptr },
    );
}

pub struct UrlRequestCallback {
    pub(crate) ptr: Cronet_UrlRequestCallbackPtr,
}

/// Users of Cronet implement this trait to receive callbacks indicating the
/// progress of a [UrlRequest] being processed.
/// An instance of this trait is passed into `URLRequest::InitWithParams()`.
///
/// Note: All methods will be invoked on the Executor passed to `URLRequest::InitWithParams()`.
impl UrlRequestCallback {
    pub fn new(handler: impl UrlRequestCallbackHandler + 'static) -> Self {
        unsafe {
            let ptr = Cronet_UrlRequestCallback_CreateWith(
                Some(cronetUrlRequestCallbackOnRedirectReceived),
                Some(cronetUrlRequestCallbackOnResponseStarted),
                Some(cronetUrlRequestCallbackOnReadCompleted),
                Some(cronetUrlRequestCallbackOnSucceeded),
                Some(cronetUrlRequestCallbackOnFailed),
                Some(cronetUrlRequestCallbackOnCanceled),
            );
            URL_REQUEST_CALLBACK_CALLBACKS
                .map()
                .lock()
                .unwrap()
                .insert(ptr, Box::new(handler));
            Self { ptr }
        }
    }
}

impl Destroy for UrlRequestCallback {
    fn destroy(&self) {
        unsafe {
            let mut lockedMap = URL_REQUEST_CALLBACK_CALLBACKS.map().lock().unwrap();
            lockedMap.remove(&self.ptr);
            Cronet_UrlRequestCallback_Destroy(self.ptr);
        }
    }
}

pub trait UrlRequestCallbackHandler {
    /// Invoked whenever a redirect is encountered. This will only be invoked
    /// between the call to `URLRequest::Start()` and
    /// `URLRequestCallbackHandler::OnResponseStarted()`.
    /// The body of the redirect response, if it has one, will be ignored.
    ///
    /// The redirect will not be followed until the `URLRequest::FollowRedirect()`
    /// method is called, either synchronously or asynchronously.
    ///
    /// Arguments:
    ///
    /// * `request`: Request being redirected.
    /// * `info`: Response information.
    /// * `newLocationUrl`: Location where the request is redirected.
    fn on_redirect_received(
        &mut self,
        url_request_callback: UrlRequestCallback,
        request: UrlRequest,
        info: UrlResponseInfo,
        new_location_url: &str,
    );

    /// Invoked when the final set of headers, after all redirects, is received.
    /// Will only be invoked once for each request.
    ///
    /// With the exception of `URLRequestCallbackHandler::OnCanceled()`,
    /// no other `URLRequestCallbackHandler` method will be invoked for the request,
    /// including `URLRequestCallbackHandler::OnSucceeded()` and
    /// `URLRequestCallbackHandler::OnFailed()`, until `URLRequest::Read()` is called to attempt
    /// to start reading the response body.
    ///
    /// Arguments:
    ///
    /// * `request`: Request that started to get the response.
    /// * `info`: Response information.
    fn on_response_started(
        &mut self,
        url_request_callback: UrlRequestCallback,
        request: UrlRequest,
        info: UrlResponseInfo,
    );

    /// Invoked whenever part of the response body has been read. Only part of
    /// the buffer may be populated, even if the entire response body has not yet
    /// been consumed. This callback transfers ownership of `buffer` back to the app,
    /// and Cronet guarantees not to access it.
    ///
    /// With the exception of `URLRequestCallbackHandler::OnCanceled()`,
    /// no other `URLRequestCallbackHandler` method will be invoked for the request,
    /// including `URLRequestCallbackHandler::OnSucceeded()` and
    /// `URLRequestCallbackHandler::OnFailed()`, until `URLRequest::Read()` is called to attempt
    /// to continue reading the response body.
    ///
    /// Arguments:
    ///
    /// * `request`: Request that received data.
    /// * `info`: Response information.
    /// * `buffer`: The buffer that was passed into `URLRequest::Read()`, now
    ///   containing the received data.
    /// * `bytesRead`: The number of bytes read into the `buffer`.
    fn on_read_completed(
        &mut self,
        url_request_callback: UrlRequestCallback,
        request: UrlRequest,
        info: UrlResponseInfo,
        buffer: Buffer,
        bytes_read: u64,
    );

    /// Invoked when the request is completed successfully. Once invoked, no other
    /// `URLRequestCallbackHandler` methods will be invoked.
    ///
    /// Implementations of `OnSucceeded()` are allowed to call
    /// `URLRequest::Destroy()`, but note that destroying the request also destroys `info`.
    ///
    /// Arguments:
    ///
    /// * `request`: Request that succeeded.
    /// * `info`: Response information. NOTE: this is owned by the request.
    fn on_succeeded(
        &mut self,
        url_request_callback: UrlRequestCallback,
        request: UrlRequest,
        info: UrlResponseInfo,
    );

    /// Invoked if the request failed for any reason after `URLRequest::Start()`.
    /// Once invoked, no other `URLRequestCallbackHandler` methods will be invoked.
    /// `error` provides information about the failure.
    ///
    /// Implementations of `URLRequestCallbackHandler::OnFailed` are allowed to call
    /// `URLRequest::Destroy()`, but note that destroying the request also destroys `info` and `error`.
    ///
    /// Arguments:
    ///
    /// * `request`: Request that failed.
    /// * `info`: Response information. May be `None` if no response was received. NOTE: this is owned by the request.
    /// * `error`: Information about the error. NOTE: this is owned by the request.
    fn on_failed(
        &mut self,
        url_request_callback: UrlRequestCallback,
        request: UrlRequest,
        info: UrlResponseInfo,
        error: CronetError,
    );

    /// Invoked if the request was canceled via `URLRequest::Cancel()`. Once
    /// invoked, no other `UrlRequestCallback` methods will be invoked.
    ///
    /// Implementations of `URLRequestCallbackHandler::OnCanceled` are allowed to call
    /// `URLRequest::Destroy()`, but note that destroying the request also destroys `info` and `error`.
    ///
    /// Arguments:
    ///
    /// * `request`: Request that was canceled.
    /// * `info`: Response information. May be `None` if no response was received. NOTE: this is owned by the request.
    fn on_canceled(
        &mut self,
        url_request_callback: UrlRequestCallback,
        request: UrlRequest,
        info: UrlResponseInfo,
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        Buffer, Destroy, UrlRequest, UrlRequestCallback, UrlRequestCallbackHandler, UrlResponseInfo,
    };

    struct TestUrlRequestCallbackHandler;

    impl UrlRequestCallbackHandler for TestUrlRequestCallbackHandler {
        fn on_redirect_received(
            &mut self,
            _: UrlRequestCallback,
            _: UrlRequest,
            _: UrlResponseInfo,
            _: &str,
        ) {
            println!("on_redirect_received");
        }

        fn on_response_started(
            &mut self,
            _: UrlRequestCallback,
            _: UrlRequest,
            _: UrlResponseInfo,
        ) {
            println!("on_response_started")
        }

        fn on_read_completed(
            &mut self,
            _: UrlRequestCallback,
            _: UrlRequest,
            _: UrlResponseInfo,
            _: Buffer,
            _: u64,
        ) {
            println!("on_read_completed");
        }

        fn on_succeeded(&mut self, _: UrlRequestCallback, _: UrlRequest, _: UrlResponseInfo) {
            println!("on_succeeded");
        }

        fn on_failed(
            &mut self,
            _: UrlRequestCallback,
            _: UrlRequest,
            _: UrlResponseInfo,
            _: crate::CronetError,
        ) {
            println!("on_failed");
        }

        fn on_canceled(&mut self, _: UrlRequestCallback, _: UrlRequest, _: UrlResponseInfo) {
            println!("on_canceled");
        }
    }

    #[test]
    fn test_url_request_callback() {
        // TODO: test handler methods
        let handler = TestUrlRequestCallbackHandler;
        let url_request_callback = UrlRequestCallback::new(handler);
        url_request_callback.destroy();
    }
}
