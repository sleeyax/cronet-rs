use std::ffi::{CStr, CString};

use once_cell::sync::Lazy;

use crate::{
    state::CronetCallbacks, Cronet_String, Cronet_UploadDataSinkPtr,
    Cronet_UploadDataSink_CreateWith, Cronet_UploadDataSink_Destroy,
    Cronet_UploadDataSink_OnReadError, Cronet_UploadDataSink_OnReadSucceeded,
    Cronet_UploadDataSink_OnRewindError, Cronet_UploadDataSink_OnRewindSucceeded, Destroy,
};

static mut UPLOAD_DATA_SINK_CALLBACKS: Lazy<
    CronetCallbacks<Cronet_UploadDataSinkPtr, UploadDataSinkCallbacks>,
> = Lazy::new(|| CronetCallbacks::new());

#[no_mangle]
unsafe extern "C" fn cronetUploadDataSinkOnReadSucceeded(
    selfPtr: Cronet_UploadDataSinkPtr,
    bytes_read: u64,
    is_final_chunk: bool,
) {
    let lockedMap = UPLOAD_DATA_SINK_CALLBACKS.map().lock().unwrap();
    if let Some(callback) = lockedMap.get(&selfPtr) {
        let on_read_succeeded = callback.on_read_succeeded;
        on_read_succeeded(UploadDataSink { ptr: selfPtr }, bytes_read, is_final_chunk);
    }
}

#[no_mangle]
unsafe extern "C" fn cronetUploadDataSinkOnReadError(
    selfPtr: Cronet_UploadDataSinkPtr,
    error_message: Cronet_String,
) {
    let lockedMap = UPLOAD_DATA_SINK_CALLBACKS.map().lock().unwrap();
    if let Some(callback) = lockedMap.get(&selfPtr) {
        let c_str = CStr::from_ptr(error_message);
        let error_message = c_str.to_str().unwrap();
        let on_read_error = callback.on_read_error;
        on_read_error(UploadDataSink { ptr: selfPtr }, error_message);
    }
}

#[no_mangle]
unsafe extern "C" fn cronetUploadDataSinkOnRewindSucceeded(selfPtr: Cronet_UploadDataSinkPtr) {
    let lockedMap = UPLOAD_DATA_SINK_CALLBACKS.map().lock().unwrap();
    if let Some(callback) = lockedMap.get(&selfPtr) {
        let on_rewind_succeeded = callback.on_rewind_succeeded;
        on_rewind_succeeded(UploadDataSink { ptr: selfPtr });
    }
}

#[no_mangle]
unsafe extern "C" fn cronetUploadDataSinkOnRewindError(
    selfPtr: Cronet_UploadDataSinkPtr,
    error_message: Cronet_String,
) {
    let lockedMap = UPLOAD_DATA_SINK_CALLBACKS.map().lock().unwrap();
    if let Some(callback) = lockedMap.get(&selfPtr) {
        let c_str = CStr::from_ptr(error_message);
        let error_message = c_str.to_str().unwrap();
        let on_rewind_error = callback.on_rewind_error;
        on_rewind_error(UploadDataSink { ptr: selfPtr }, error_message);
    }
}

#[derive(Clone, Copy)]
pub struct UploadDataSinkCallbacks {
    pub on_read_succeeded: OnReadSucceededFn,
    pub on_read_error: OnReadErrorFn,
    pub on_rewind_succeeded: OnRewindSucceededFn,
    pub on_rewind_error: OnRewindErrorFn,
}

pub type OnReadSucceededFn = fn(callback: UploadDataSink, bytes_read: u64, is_final_chunk: bool);
pub type OnReadErrorFn = fn(callback: UploadDataSink, error_message: &str);
pub type OnRewindSucceededFn = fn(callback: UploadDataSink);
pub type OnRewindErrorFn = fn(callback: UploadDataSink, error_message: &str);

/// Defines callbacks methods for [crate::UploadDataProvider].
/// All methods may be called synchronously or asynchronously, on any thread.
pub struct UploadDataSink {
    pub(crate) ptr: Cronet_UploadDataSinkPtr,
}

impl UploadDataSink {
    pub fn new(callbacks: UploadDataSinkCallbacks) -> Self {
        unsafe {
            let ptr = Cronet_UploadDataSink_CreateWith(
                Some(cronetUploadDataSinkOnReadSucceeded),
                Some(cronetUploadDataSinkOnReadError),
                Some(cronetUploadDataSinkOnRewindSucceeded),
                Some(cronetUploadDataSinkOnRewindError),
            );
            UPLOAD_DATA_SINK_CALLBACKS
                .map()
                .lock()
                .unwrap()
                .insert(ptr, callbacks);
            Self { ptr }
        }
    }

    /// Called by `UploadDataProviderHandler` when a read succeeds.
    ///
    /// Arguments:
    ///
    /// * `bytesRead`: Number of bytes read into the buffer passed to `UploadDataProviderHandler.Read()`.
    /// * `finalChunk`: For chunked uploads, `true` if this is the final read. It must be `false` for non-chunked uploads.
    pub fn on_read_succeeded(&self, bytes_read: u64, is_final_chunk: bool) {
        unsafe {
            Cronet_UploadDataSink_OnReadSucceeded(self.ptr, bytes_read, is_final_chunk);
        }
    }

    /// Called by `UploadDataProviderHandler` when a read fails.
    ///
    /// Arguments:
    ///
    /// * `message`: Message to pass on to `URLRequestCallbackHandler::OnFailed()`.
    pub fn on_read_error(&self, error_message: &str) {
        unsafe {
            let c_str = CString::new(error_message).unwrap();
            Cronet_UploadDataSink_OnReadError(self.ptr, c_str.as_ptr());
        }
    }

    /// Called by [crate::UploadDataProviderHandler] when a rewind succeeds.
    pub fn on_rewind_succeeded(&self) {
        unsafe {
            Cronet_UploadDataSink_OnRewindSucceeded(self.ptr);
        }
    }

    /// Called by [crate::UploadDataProviderHandler] when a rewind fails, or if rewinding uploads is not supported.
    pub fn on_rewind_error(&self, error_message: &str) {
        unsafe {
            let c_str = CString::new(error_message).unwrap();
            Cronet_UploadDataSink_OnRewindError(self.ptr, c_str.as_ptr());
        }
    }
}

impl Destroy for UploadDataSink {
    fn destroy(&self) {
        unsafe {
            let mut lockedMap = UPLOAD_DATA_SINK_CALLBACKS.map().lock().unwrap();
            lockedMap.remove(&self.ptr);
            Cronet_UploadDataSink_Destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Destroy, UploadDataSink, UploadDataSinkCallbacks};

    #[test]
    fn test_upload_data_sink() {
        // TODO: assert that callbacks are actually called
        let upload_data_sink = UploadDataSink::new(UploadDataSinkCallbacks {
            on_read_succeeded: |_, _, _| println!("on_read_succeeded"),
            on_read_error: |_, _| println!("on_read_error"),
            on_rewind_succeeded: |_| println!("on_rewind_succeeded"),
            on_rewind_error: |_, _| println!("on_rewind_error"),
        });
        upload_data_sink.on_read_succeeded(10, true);
        upload_data_sink.on_rewind_succeeded();
        upload_data_sink.on_read_error("error");
        upload_data_sink.on_rewind_error("error");
        upload_data_sink.destroy();
    }
}
