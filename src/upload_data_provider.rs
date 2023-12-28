use once_cell::sync::Lazy;

use crate::{
    state::CronetCallbacks, Buffer, Cronet_BufferPtr, Cronet_ClientContext,
    Cronet_UploadDataProviderPtr, Cronet_UploadDataProvider_Close,
    Cronet_UploadDataProvider_CreateWith, Cronet_UploadDataProvider_Destroy,
    Cronet_UploadDataProvider_GetClientContext, Cronet_UploadDataProvider_GetLength,
    Cronet_UploadDataProvider_Read, Cronet_UploadDataProvider_Rewind,
    Cronet_UploadDataProvider_SetClientContext, Cronet_UploadDataSinkPtr, Destroy, UploadDataSink,
};

static mut UPLOAD_DATA_PROVIDER_CALLBACKS: Lazy<
    CronetCallbacks<Cronet_UploadDataProviderPtr, Box<dyn UploadDataProviderHandler>>,
> = Lazy::new(CronetCallbacks::new);

#[no_mangle]
unsafe extern "C" fn cronetUploadDataProviderGetLength(
    selfPtr: Cronet_UploadDataProviderPtr,
) -> i64 {
    let lockedMap = UPLOAD_DATA_PROVIDER_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get(&selfPtr).unwrap();
    callback.length(UploadDataProvider { ptr: selfPtr })
}

#[no_mangle]
unsafe extern "C" fn cronetUploadDataProviderRead(
    self_ptr: Cronet_UploadDataProviderPtr,
    upload_data_sink_ptr: Cronet_UploadDataSinkPtr,
    buffer_ptr: Cronet_BufferPtr,
) {
    let lockedMap = UPLOAD_DATA_PROVIDER_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get(&self_ptr).unwrap();
    callback.read(
        UploadDataProvider { ptr: self_ptr },
        UploadDataSink {
            ptr: upload_data_sink_ptr,
        },
        Buffer { ptr: buffer_ptr },
    );
}

#[no_mangle]
unsafe extern "C" fn cronetUploadDataProviderRewind(
    self_ptr: Cronet_UploadDataProviderPtr,
    upload_data_sink_ptr: Cronet_UploadDataSinkPtr,
) {
    let mut lockedMap = UPLOAD_DATA_PROVIDER_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get_mut(&self_ptr).unwrap();
    callback.rewind(
        UploadDataProvider { ptr: self_ptr },
        UploadDataSink {
            ptr: upload_data_sink_ptr,
        },
    );
}

#[no_mangle]
unsafe extern "C" fn cronetUploadDataProviderClose(selfPtr: Cronet_UploadDataProviderPtr) {
    let lockedMap = UPLOAD_DATA_PROVIDER_CALLBACKS.map().lock().unwrap();
    let callback = lockedMap.get(&selfPtr).unwrap();
    callback.close(UploadDataProvider { ptr: selfPtr });
}

pub struct UploadDataProvider {
    pub(crate) ptr: Cronet_UploadDataProviderPtr,
}

impl UploadDataProvider {
    pub fn new(handler: impl UploadDataProviderHandler + 'static) -> Self {
        unsafe {
            let ptr = Cronet_UploadDataProvider_CreateWith(
                Some(cronetUploadDataProviderGetLength),
                Some(cronetUploadDataProviderRead),
                Some(cronetUploadDataProviderRewind),
                Some(cronetUploadDataProviderClose),
            );
            UPLOAD_DATA_PROVIDER_CALLBACKS
                .map()
                .lock()
                .unwrap()
                .insert(ptr, Box::new(handler));
            Self { ptr }
        }
    }

    pub fn length(&self) -> i64 {
        unsafe { Cronet_UploadDataProvider_GetLength(self.ptr) }
    }

    pub fn read(&self, upload_data_sink: UploadDataSink, buffer: Buffer) {
        unsafe {
            Cronet_UploadDataProvider_Read(self.ptr, upload_data_sink.ptr, buffer.ptr);
        }
    }

    pub fn rewind(&self, upload_data_sink: UploadDataSink) {
        unsafe {
            Cronet_UploadDataProvider_Rewind(self.ptr, upload_data_sink.ptr);
        }
    }

    pub fn close(&self) {
        unsafe {
            Cronet_UploadDataProvider_Close(self.ptr);
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn set_client_context(&self, client_context: Cronet_ClientContext) {
        Cronet_UploadDataProvider_SetClientContext(self.ptr, client_context)
    }

    pub fn client_context(&self) -> Cronet_ClientContext {
        unsafe { Cronet_UploadDataProvider_GetClientContext(self.ptr) }
    }
}

impl Destroy for UploadDataProvider {
    fn destroy(&self) {
        unsafe { Cronet_UploadDataProvider_Destroy(self.ptr) }
    }
}

pub trait UploadDataProviderHandler {
    /// If this is a non-chunked upload, returns the length of the upload.
    /// Must always return -1 if this is a chunked upload.
    fn length(&self, upload_data_provider: UploadDataProvider) -> i64;

    /// Reads upload data into `buffer`. Each call of this method must be followed by a
    /// single call, either synchronous or asynchronous, to
    /// `UploadDataSink::OnReadSucceeded()` on success
    /// or `UploadDataSink::OnReadError()` on failure. Neither read nor rewind
    /// will be called until one of those methods or the other is called. Even if
    /// the associated `UrlRequest` is canceled, one or the other must
    /// still be called before resources can be safely freed.
    ///
    /// Arguments:
    ///
    /// * `sink`: The object to notify when the read has completed, successfully or otherwise.
    /// * `buffer`: The buffer to copy the read bytes into.
    fn read(&self, upload_data_provider: UploadDataProvider, sink: UploadDataSink, buffer: Buffer);

    /// Rewinds upload data. Each call must be followed by a single
    /// call, either synchronous or asynchronous, to
    /// `UploadDataSink::OnRewindSucceeded()` on success or
    /// `UploadDataSink::OnRewindError()` on failure. Neither read nor rewind
    /// will be called until one of those methods or the other is called.
    /// Even if the associated `UrlRequest` is canceled, one or the other
    /// must still be called before resources can be safely freed.
    ///
    /// If rewinding is not supported, this should call
    /// `UploadDataSink::OnRewindError()`. Note that rewinding is required to
    /// follow redirects that preserve the upload body and for retrying when the
    /// server times out stale sockets.
    ///
    /// Arguments:
    ///
    /// * `sink`: The object to notify when the rewind operation has completed, successfully or otherwise.
    fn rewind(&mut self, upload_data_provider: UploadDataProvider, sink: UploadDataSink);

    /// Called when this [UploadDataProvider] is no longer needed by a request, so that resources (like a file) can be explicitly released.
    fn close(&self, upload_data_provider: UploadDataProvider);
}

#[cfg(test)]
mod tests {
    use crate::{
        Buffer, Destroy, UploadDataProvider, UploadDataProviderHandler, UploadDataSink,
        UploadDataSinkCallbacks,
    };

    struct TestUploadDataProviderHandler;

    impl UploadDataProviderHandler for TestUploadDataProviderHandler {
        fn length(&self, _: UploadDataProvider) -> i64 {
            10
        }

        fn read(&self, _: UploadDataProvider, sink: UploadDataSink, buffer: Buffer) {
            let size = buffer.size();
            sink.on_read_succeeded(size, false);
        }

        fn rewind(&mut self, _: UploadDataProvider, sink: UploadDataSink) {
            sink.on_rewind_succeeded();
        }

        fn close(&self, upload_data_provider: UploadDataProvider) {
            upload_data_provider.destroy();
        }
    }

    #[test]
    fn test_upload_data_provider() {
        // TODO: test that callbacks are actually called
        let handler = TestUploadDataProviderHandler;
        let upload_data_provider = UploadDataProvider::new(handler);
        assert_eq!(upload_data_provider.length(), 10);
        let buffer = Buffer::new_with_size(10);
        let callbacks = UploadDataSinkCallbacks {
            on_read_succeeded: |_, _, _| {},
            on_read_error: |_, _| {},
            on_rewind_succeeded: |_| {},
            on_rewind_error: |_, _| {},
        };
        upload_data_provider.read(UploadDataSink::new(callbacks), buffer);
        upload_data_provider.rewind(UploadDataSink::new(callbacks));
        upload_data_provider.destroy();
    }
}
