use std::ptr;

use crate::{Buffer, Destroy, UploadDataProvider, UploadDataProviderHandler, UploadDataSink};

use super::Body;

type RewindFn = dyn Fn() -> Body;

pub struct BodyUploadDataProvider<'a> {
    /// The HTTP request body to be uploaded.
    body: Body,

    /// A function that returns a new body to be used for rewinding.
    /// This is used for retrying requests when the server times out stale sockets.
    /// Note that rewinding is also required to follow redirects that preserve the upload body.
    /// If this is `None`, rewinding is not supported.
    rewind: Option<&'a RewindFn>,
}

impl<'a> UploadDataProviderHandler for BodyUploadDataProvider<'a> {
    fn length(&self, _: UploadDataProvider) -> i64 {
        self.body.len().unwrap_or(0) as i64
    }

    fn read(&self, _: UploadDataProvider, sink: UploadDataSink, buffer: Buffer) {
        if let Some(bytes) = self.body.as_bytes() {
            let ptr = buffer.data_ptr();
            let len = buffer.size();
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, len as usize);
            }
            sink.on_read_succeeded(len, true); // TODO: implement chunked reads
        } else {
            sink.on_read_error("Empty body");
        }
    }

    fn rewind(&mut self, _: UploadDataProvider, sink: UploadDataSink) {
        if let Some(rewind) = &self.rewind {
            self.body = rewind();
            sink.on_rewind_succeeded();
        } else {
            sink.on_rewind_error("Rewinding is not supported");
        }
    }

    fn close(&self, upload_data_provider: UploadDataProvider) {
        upload_data_provider.destroy();
    }
}

impl<'a> BodyUploadDataProvider<'a> {
    pub fn new(body: Body, rewind: Option<&'a RewindFn>) -> Self {
        Self { body, rewind }
    }
}
